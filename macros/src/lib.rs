use proc_macro2::{Literal, TokenStream, TokenTree};
use proc_macro_crate::FoundCrate;
use quote::{format_ident, quote};
use std::{
    any::Any,
    env, fs,
    io::{self, Write},
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
#[proc_macro]
pub fn embed_dir(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand(input.into())
        .unwrap_or_else(|message| {
            let message = format!("embed_dir!: {message}");
            quote!(::core::compile_error!(#message))
        })
        .into()
}

fn expand(input: TokenStream) -> Result<TokenStream> {
    let options = Options::parse(input)?;

    let manifest = env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| "CARGO_MANIFEST_DIR is not set".to_owned())?;
    let root = PathBuf::from(manifest).join(&options.path);
    if !root.is_dir() {
        return Err(format!("directory does not exist: {}", root.display()));
    }

    let krate = match proc_macro_crate::crate_name("sawe")
        .map_err(|e| format!("`sawe` must be a dependency: {e}"))?
    {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let name = format_ident!("{}", name);
            quote!(::#name)
        }
    };

    let prefix = root.file_name().filter(|_| options.show_root).unwrap_or_default();
    let mut items = Vec::new();
    collect(&root, Path::new(prefix), &mut items)?;

    // Each compressed buffer is consumed (and freed) as soon as its literal exists.
    let packed = pack_all(&items, options.profile)?;
    let mut entries = TokenStream::new();
    for (item, packed) in items.into_iter().zip(packed) {
        entries.extend(entry(item, packed, &krate, options.profile));
    }

    Ok(quote!(#krate::fsread::Directory { entries: vec![#entries] }))
}

// ---------------------------------------------------------------------------
// Argument parsing (path first, remaining options in any order)
// ---------------------------------------------------------------------------

struct Options {
    path: String,
    show_root: bool,
    profile: Profile,
}

impl Options {
    fn parse(input: TokenStream) -> Result<Self> {
        let mut segments = split_commas(input).into_iter();
        let path = parse_path(segments.next().as_deref())?;

        let (mut show_root, mut profile) = (None, None);
        for segment in segments {
            match segment.as_slice() {
                [] => {}
                [TokenTree::Literal(_)] => {
                    return Err("unexpected string literal; the path must be the first argument and can only be given once".into());
                }
                [TokenTree::Ident(i)] if i == "true" || i == "false" => {
                    set_once(&mut show_root, i == "true", "show_root flag")?;
                }
                // `Compression::Balanced` or `Balanced`: only the last identifier counts.
                tokens => {
                    let value = Profile::from_name(&profile_name(tokens)?)?;
                    set_once(&mut profile, value, "compression profile")?;
                }
            }
        }

        Ok(Options {
            path,
            show_root: show_root.unwrap_or(false),
            profile: profile.unwrap_or(Profile::None),
        })
    }
}

fn parse_path(segment: Option<&[TokenTree]>) -> Result<String> {
    let Some([TokenTree::Literal(lit)]) = segment else {
        return Err("the first argument must be the directory path as a string literal".into());
    };
    let text = lit.to_string();
    text.strip_prefix('"')
        .and_then(|t| t.strip_suffix('"'))
        .filter(|t| !t.is_empty() && !t.contains('\\'))
        .map(str::to_owned)
        .ok_or_else(|| {
            format!("the path must be a plain, non-empty string literal without escapes (use `/` as separator), got {text}")
        })
}

fn split_commas(input: TokenStream) -> Vec<Vec<TokenTree>> {
    let (mut segments, mut current) = (Vec::new(), Vec::new());
    for token in input {
        match token {
            TokenTree::Punct(p) if p.as_char() == ',' => segments.push(std::mem::take(&mut current)),
            token => current.push(token),
        }
    }
    segments.push(current);
    segments
}

fn profile_name(tokens: &[TokenTree]) -> Result<String> {
    let well_formed = tokens.iter().all(|t| match t {
        TokenTree::Ident(_) => true,
        TokenTree::Punct(p) => p.as_char() == ':',
        _ => false,
    });

    match tokens.last() {
        Some(TokenTree::Ident(last)) if well_formed => Ok(last.to_string()),
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Profile {
    None,
    Fast,
    Balanced,
    Best,
    Maximum,
}

impl Profile {
    fn from_name(name: &str) -> Result<Self> {
        Ok(match name {
            "None" => Self::None,
            "Fast" => Self::Fast,
            "Balanced" => Self::Balanced,
            "Best" => Self::Best,
            "Maximum" => Self::Maximum,
            other => {
                return Err(format!(
                    "unknown compression profile `{other}`; use None, Fast, Balanced, Best, or Maximum"
                ))
            }
        })
    }

    fn codec(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Fast => "Lz4",
            Self::Balanced | Self::Best => "Zstd",
            Self::Maximum => "Brotli",
        }
    }

    fn compress(self, data: &[u8]) -> Result<Vec<u8>> {
        fn zstd(data: &[u8], level: i32) -> Result<Vec<u8>> {
            zstd::bulk::compress(data, level).map_err(|e| format!("zstd compression failed: {e}"))
        }

        match self {
            Self::None => Err("no compressor for profile `None`".into()),
            Self::Fast => Ok(lz4_flex::compress_prepend_size(data)),
            Self::Balanced => zstd(data, 3),
            Self::Best => zstd(data, 15),
            Self::Maximum => {
                let mut encoder = brotli::CompressorWriter::new(Vec::new(), 4096, 11, 22);
                encoder
                    .write_all(data)
                    .map_err(|e| format!("brotli compression failed: {e}"))?;
                Ok(encoder.into_inner())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Directory walk
// ---------------------------------------------------------------------------

/// A directory (`file == None`) or a file with its canonical absolute path.
/// Paths are checked for UTF-8 once, here, so code generation cannot fail.
struct Item {
    relative: String,
    file: Option<String>,
}

fn utf8(path: &Path) -> Result<String> {
    path.to_str()
        .map(str::to_owned)
        .ok_or_else(|| format!("path is not valid UTF-8: {}", path.display()))
}

/// Depth-first, sorted walk: a directory entry comes before its children.
fn collect(dir: &Path, rel: &Path, items: &mut Vec<Item>) -> Result<()> {
    let mut names = fs::read_dir(dir)
        .map_err(|e| format!("cannot read {}: {e}", dir.display()))?
        .map(|entry| entry.map(|e| e.file_name()))
        .collect::<io::Result<Vec<_>>>()
        .map_err(|e| format!("cannot read an entry of {}: {e}", dir.display()))?;
    names.sort_unstable();

    for name in names {
        let path = dir.join(&name);
        let rel = rel.join(&name);

        // Follows symlinks; anything unreadable or neither file nor dir is skipped.
        let Ok(meta) = fs::metadata(&path) else { continue };
        if meta.is_dir() {
            items.push(Item { relative: utf8(&rel)?, file: None });
            collect(&path, &rel, items)?;
        } else if meta.is_file() {
            let absolute = fs::canonicalize(&path)
                .map_err(|e| format!("cannot resolve {}: {e}", path.display()))?;
            items.push(Item { relative: utf8(&rel)?, file: Some(utf8(&absolute)?) });
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Compression driver (workers return plain bytes: proc_macro types are
// main-thread only) and code generation
// ---------------------------------------------------------------------------

/// `Some(bytes)` if the file is stored compressed, `None` if it is embedded as is.
fn pack(item: &Item, profile: Profile) -> Result<Option<Vec<u8>>> {
    let Some(file) = item.file.as_deref().filter(|_| profile != Profile::None) else {
        return Ok(None);
    };
    let data = fs::read(file).map_err(|e| format!("cannot read {file}: {e}"))?;
    let mut packed = profile.compress(&data).map_err(|e| format!("{file}: {e}"))?;

    if packed.len() >= data.len() {
        return Ok(None);
    }
    packed.shrink_to_fit();
    Ok(Some(packed))
}

/// Packs all items in order, on several threads when a compression profile is
/// active. Worker failures, including panics, come back as an error message.
fn pack_all(items: &[Item], profile: Profile) -> Result<Vec<Option<Vec<u8>>>> {
    let threads = match profile {
        Profile::None => 1,
        _ => thread::available_parallelism().map_or(1, |n| n.get()),
    }
    .min(items.len());

    if threads <= 1 {
        return items.iter().map(|item| pack(item, profile)).collect();
    }

    thread::scope(|scope| {
        let workers = items
            .chunks(items.len().div_ceil(threads))
            .map(|chunk| {
                thread::Builder::new()
                    .stack_size(WORKER_STACK)
                    .spawn_scoped(scope, move || {
                        chunk.iter().map(|item| pack(item, profile)).collect::<Result<Vec<_>>>()
                    })
                    .map_err(|e| format!("cannot start a worker thread: {e}"))
            })
            .collect::<Result<Vec<_>>>()?;

        let mut packed = Vec::with_capacity(items.len());
        for worker in workers {
            let part = worker
                .join()
                .map_err(|p| format!("worker thread panicked: {}", panic_message(&*p)))??;
            packed.extend(part);
        }
        Ok(packed)
    })
}

fn entry(item: Item, packed: Option<Vec<u8>>, _crate: &TokenStream, profile: Profile) -> TokenStream {
    let path = item.relative;
    let Some(file) = item.file else {
        return quote!(#_crate::fsread::Entry::embedded_dir(::std::path::PathBuf::from(#path)),);
    };

    let (data, codec) = match packed {
        None => (quote!(include_bytes!(#file)), "None"),
        Some(bytes) => {
            let bytes = Literal::byte_string(&bytes);
            // The compressed bytes are a literal, so cargo cannot track the original.
            // The unused const (not part of the final binary) depends on it instead.
            let data = if TRACK_ORIGINALS {
                quote!({ const _: &[u8] = include_bytes!(#file); #bytes })
            } else {
                quote!(#bytes)
            };
            (data, profile.codec())
        }
    };
    let codec = format_ident!("{}", codec);

    quote!(#_crate::fsread::Entry::embedded_file(
        ::std::path::PathBuf::from(#path),
        #data,
        #_crate::fsread::Codec::#codec
    ),)
}

fn panic_message(payload: &(dyn Any + Send)) -> &str {
    payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
        .unwrap_or("unknown panic")
}