use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use std::{
    any::Any,
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    thread,
};

type Result<T> = std::result::Result<T, String>;


const TRACK_ORIGINALS: bool = true;
const WORKER_STACK: usize = 16 * 1024 * 1024;

/// Embeds a directory into the binary.
///
/// The path always comes first; the options after it may be given in any order.
///
/// | position | option                         | meaning                                       | default  |
/// |----------|--------------------------------|-----------------------------------------------|----------|
/// | first    | `"assets"` (string literal)    | directory, relative to `CARGO_MANIFEST_DIR`   | required |
/// | any      | `true` / `false`               | keep the directory name as path prefix        | `false`  |
/// | any      | `Compression::X` (or just `X`) | `None`, `Fast`, `Balanced`, `Best`, `Maximum` | `None`   |
///
/// ```ignore
/// embed_dir!("assets");
/// embed_dir!("assets", true);
/// embed_dir!("assets", Compression::Balanced, true);
/// embed_dir!("assets", true, Compression::Best);
/// ```
///
#[proc_macro]
pub fn embed_dir(input: TokenStream) -> TokenStream {
    match expand(input) {
        Ok(tokens) => tokens,
        Err(message) => compile_error(&message),
    }
}

fn compile_error(message: &str) -> TokenStream {
    // `{:?}` of a `String` is always a valid Rust string literal.
    format!("::core::compile_error!({:?})", format!("embed_dir!: {message}"))
        .parse()
        .expect("compile_error! invocation is always valid")
}

fn expand(input: TokenStream) -> Result<TokenStream> {
    let options = Options::parse(input)?;

    let manifest = env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| "CARGO_MANIFEST_DIR is not set".to_owned())?;
    let root = PathBuf::from(manifest).join(&options.path);

    if !root.is_dir() {
        return Err(format!("directory does not exist: {}", root.display()));
    }

    let crate_path = match proc_macro_crate::crate_name("sawe")
        .map_err(|e| format!("`sawe` must be a dependency: {e}"))?
    {
        proc_macro_crate::FoundCrate::Itself => "crate".to_string(),
        proc_macro_crate::FoundCrate::Name(name) => format!("::{name}"),
    };

    let root_name = root
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let mut items = Vec::new();
    collect(&root, &root, &root_name, options.show_root, &mut items)?;

    let rendered = render_all(&items, &options.profile, &crate_path)?;

    // Turn each rendered entry into tokens and drop its text right away, so
    // the generated source never exists a second time as one big string.
    let mut entries = TokenStream::new();
    for text in rendered {
        let tokens: TokenStream = text
            .parse()
            .map_err(|e| format!("failed to generate Rust code: {e}"))?;
        entries.extend(std::iter::once(tokens));
    }

    // `<crate>::fsread::Directory { entries: vec![ <entries> ] }`
    let mut body: TokenStream = "entries: vec!"
        .parse()
        .map_err(|e| format!("failed to generate Rust code: {e}"))?;
    body.extend([TokenTree::Group(Group::new(Delimiter::Bracket, entries))]);

    let mut output: TokenStream = format!("{crate_path}::fsread::Directory")
        .parse()
        .map_err(|e| format!("failed to generate Rust code: {e}"))?;
    output.extend([TokenTree::Group(Group::new(Delimiter::Brace, body))]);

    Ok(output)
}

// ---------------------------------------------------------------------------
// Argument parsing (path first, remaining options in any order)
// ---------------------------------------------------------------------------

struct Options {
    path: String,
    show_root: bool,
    profile: String,
}

impl Options {
    fn parse(input: TokenStream) -> Result<Self> {
        let mut segments = split_commas(input).into_iter();

        // The path is always the first argument.
        let path = match segments.next().as_deref() {
            Some([TokenTree::Literal(lit)]) => {
                let text = lit.to_string();
                text.strip_prefix('"')
                    .and_then(|t| t.strip_suffix('"'))
                    .filter(|t| !t.is_empty() && !t.contains('\\'))
                    .ok_or_else(|| {
                        format!(
                            "the path must be a plain, non-empty string literal without \
                             escapes (use `/` as separator), got {text}"
                        )
                    })?
                    .to_owned()
            }
            _ => return Err("the first argument must be the directory path as a string literal".into()),
        };

        // Everything after the path is optional and order independent.
        let mut show_root = None;
        let mut profile = None;

        for segment in segments {
            match segment.as_slice() {
                [] => {}
                [TokenTree::Literal(_)] => {
                    return Err("unexpected string literal; the path must be the first argument and can only be given once".into());
                }
                [TokenTree::Ident(ident)] if matches!(ident.to_string().as_str(), "true" | "false") => {
                    set_once(&mut show_root, ident.to_string() == "true", "show_root flag")?;
                }
                tokens => {
                    // `Compression::Balanced` or `Balanced`: only the last identifier counts.
                    let name = profile_name(tokens)?;
                    codec_for(&name)?;
                    set_once(&mut profile, name, "compression profile")?;
                }
            }
        }

        Ok(Options {
            path,
            show_root: show_root.unwrap_or(false),
            profile: profile.unwrap_or_else(|| "None".to_owned()),
        })
    }
}

fn split_commas(input: TokenStream) -> Vec<Vec<TokenTree>> {
    let mut segments = vec![Vec::new()];
    for token in input {
        match &token {
            TokenTree::Punct(p) if p.as_char() == ',' => segments.push(Vec::new()),
            _ => segments.last_mut().expect("always one segment").push(token),
        }
    }
    segments
}

fn profile_name(tokens: &[TokenTree]) -> Result<String> {
    let well_formed = tokens.iter().all(|t| match t {
        TokenTree::Ident(_) => true,
        TokenTree::Punct(p) => p.as_char() == ':',
        _ => false,
    });

    match (well_formed, tokens.last()) {
        (true, Some(TokenTree::Ident(last))) => Ok(last.to_string()),
        _ => Err(format!(
            "unrecognised option `{}`; expected `true`/`false` or a compression profile \
             (the path must be the first argument)",
            tokens.iter().map(ToString::to_string).collect::<String>()
        )),
    }
}

fn set_once<T>(slot: &mut Option<T>, value: T, what: &str) -> Result<()> {
    match slot.replace(value) {
        None => Ok(()),
        Some(_) => Err(format!("{what} given more than once")),
    }
}

// ---------------------------------------------------------------------------
// Compression
// ---------------------------------------------------------------------------

fn codec_for(profile: &str) -> Result<&'static str> {
    match profile {
        "None" => Ok("None"),
        "Fast" => Ok("Lz4"),
        "Balanced" | "Best" => Ok("Zstd"),
        "Maximum" => Ok("Brotli"),
        other => Err(format!(
            "unknown compression profile `{other}`; use None, Fast, Balanced, Best, or Maximum"
        )),
    }
}

fn compress(data: &[u8], profile: &str) -> Result<Vec<u8>> {
    match profile {
        "Fast" => Ok(lz4_flex::compress_prepend_size(data)),
        "Balanced" => zstd::bulk::compress(data, 3).map_err(|e| format!("zstd compression failed: {e}")),
        "Best" => zstd::bulk::compress(data, 15).map_err(|e| format!("zstd compression failed: {e}")),
        "Maximum" => {
            let mut encoder = brotli::CompressorWriter::new(Vec::new(), 4096, 11, 22);
            encoder
                .write_all(data)
                .map_err(|e| format!("brotli compression failed: {e}"))?;
            Ok(encoder.into_inner())
        }
        other => Err(format!("no compressor for profile `{other}`")),
    }
}

// ---------------------------------------------------------------------------
// Directory walk
// ---------------------------------------------------------------------------

enum Item {
    Dir { relative: PathBuf },
    File { relative: PathBuf, absolute: PathBuf },
}

/// Depth-first, sorted walk: a directory entry comes before its children.
fn collect(
    root: &Path,
    current: &Path,
    root_name: &str,
    show_root: bool,
    items: &mut Vec<Item>,
) -> Result<()> {
    let mut children = Vec::new();
    for entry in fs::read_dir(current)
        .map_err(|e| format!("cannot read {}: {e}", current.display()))?
    {
        let entry = entry.map_err(|e| format!("cannot read an entry of {}: {e}", current.display()))?;
        children.push(entry.path());
    }

    children.sort();

    for path in children {
        let relative = path
            .strip_prefix(root)
            .map_err(|_| format!("{} is outside the embedding root", path.display()))?;

        let relative = if show_root {
            Path::new(root_name).join(relative)
        } else {
            relative.to_path_buf()
        };

        if path.is_dir() {
            items.push(Item::Dir { relative });
            collect(root, &path, root_name, show_root, items)?;
        } else if path.is_file() {
            let absolute = fs::canonicalize(&path)
                .map_err(|e| format!("cannot resolve {}: {e}", path.display()))?;
            items.push(Item::File { relative, absolute });
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Code generation (std only: this runs on worker threads)
// ---------------------------------------------------------------------------

/// A valid Rust string literal for a path.
fn path_literal(path: &Path) -> Result<String> {
    let text = path
        .to_str()
        .ok_or_else(|| format!("path is not valid UTF-8: {}", path.display()))?;
    Ok(format!("{text:?}"))
}

fn render(item: &Item, profile: &str, crate_path: &str) -> Result<String> {
    match item {
        Item::Dir { relative } => Ok(format!(
            "{crate_path}::fsread::Entry::embedded_dir(\
                ::std::path::PathBuf::from({})\
            ),",
            path_literal(relative)?
        )),
        Item::File { relative, absolute } => render_file(relative, absolute, profile, crate_path),
    }
}

fn render_file(
    relative: &Path,
    absolute: &Path,
    profile: &str,
    crate_path: &str,
) -> Result<String> {
    let head = format!(
        "{crate_path}::fsread::Entry::embedded_file(::std::path::PathBuf::from({}),",
        path_literal(relative)?
    );
    let include = format!("include_bytes!({})", path_literal(absolute)?);
    let tail = |codec: &str| format!(",{crate_path}::fsread::Codec::{codec}),");

    let codec = codec_for(profile)?;
    if codec == "None" {
        return Ok(format!("{head}{include}{}", tail("None")));
    }

    let data = fs::read(absolute)
        .map_err(|e| format!("cannot read {}: {e}", absolute.display()))?;
    let packed = compress(&data, profile)
        .map_err(|e| format!("{}: {e}", absolute.display()))?;

    if packed.len() >= data.len() {
        return Ok(format!("{head}{include}{}", tail("None")));
    }
    drop(data);

    // The compressed bytes are a literal, so cargo cannot track the original.
    // The unused const (not part of the final binary) depends on it instead.
    let (open, close) = if TRACK_ORIGINALS {
        (format!("{{ const _: &[u8] = {include}; "), " }")
    } else {
        (String::new(), "")
    };
    let tail = tail(codec);

    // Exact capacity: the literal is written once, with no regrowth copies.
    let mut out = String::with_capacity(
        head.len() + open.len() + byte_string_len(&packed) + close.len() + tail.len(),
    );
    out.push_str(&head);
    out.push_str(&open);
    push_byte_string(&mut out, &packed);
    out.push_str(close);
    out.push_str(&tail);
    Ok(out)
}

/// Printable ASCII other than `"` and `\` is written as is, everything else
/// as `\xNN`.
fn is_plain(byte: u8) -> bool {
    (0x20..0x7f).contains(&byte) && byte != b'"' && byte != b'\\'
}

/// Length of `b"..."` for `bytes`, including `b` and both quotes.
fn byte_string_len(bytes: &[u8]) -> usize {
    3 + bytes.iter().map(|&b| if is_plain(b) { 1 } else { 4 }).sum::<usize>()
}

fn push_byte_string(out: &mut String, bytes: &[u8]) {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    out.push_str("b\"");
    for &byte in bytes {
        if is_plain(byte) {
            out.push(byte as char);
        } else {
            out.push_str("\\x");
            out.push(HEX[(byte >> 4) as usize] as char);
            out.push(HEX[(byte & 0x0f) as usize] as char);
        }
    }
    out.push('"');
}

/// Renders all items in order, compressing on several threads when a
/// compression profile is active. Worker failures, including panics, come back
/// as the original error message.
fn render_all(items: &[Item], profile: &str, crate_path: &str) -> Result<Vec<String>> {
    let threads = if profile == "None" {
        1
    } else {
        thread::available_parallelism().map_or(1, |n| n.get())
    }
    .min(items.len());

    if threads <= 1 {
        return items.iter().map(|i| render(i, profile, crate_path)).collect();
    }

    let chunk_size = items.len().div_ceil(threads);

    thread::scope(|scope| {
        let mut workers = Vec::with_capacity(threads);

        for chunk in items.chunks(chunk_size) {
            let worker = thread::Builder::new()
                .stack_size(WORKER_STACK)
                .spawn_scoped(scope, move || {
                    chunk
                        .iter()
                        .map(|i| render(i, profile, crate_path))
                        .collect::<Result<Vec<_>>>()
                })
                .map_err(|e| format!("cannot start a worker thread: {e}"))?;
            workers.push(worker);
        }

        let mut rendered = Vec::with_capacity(items.len());
        for worker in workers {
            let part = worker
                .join()
                .map_err(|payload| format!("worker thread panicked: {}", panic_message(&*payload)))??;
            rendered.extend(part);
        }
        Ok(rendered)
    })
}

fn panic_message(payload: &(dyn Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|s| (*s).to_owned())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "unknown panic".to_owned())
}