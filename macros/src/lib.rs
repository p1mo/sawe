use proc_macro::TokenStream;
use std::{
    env,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

/// Usage: embed_dir!("assets", true, Compression::Balanced)
#[proc_macro]
pub fn embed_dir(input: TokenStream) -> TokenStream {
    let args = input.to_string();
    let parts: Vec<&str> = args.split(',').map(str::trim).collect();

    if parts.is_empty() {
        panic!("embed_dir!: expected a directory path");
    }

    let path = parts[0].trim_matches('"');
    if path == parts[0] || path.is_empty() {
        panic!("embed_dir!: first argument must be a string literal");
    }

    let show_root = parts.get(1).map(|s| *s == "true").unwrap_or(false);
    let profile = parts.get(2)
        .map(|s| s.rsplit("::").next().unwrap_or(s).trim().to_owned())
        .unwrap_or_else(|| "None".to_owned());

    let manifest = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR missing")
    );
    let root = manifest.join(path);

    if !root.is_dir() {
        panic!("embed_dir!: directory does not exist: {}", root.display());
    }

    let crate_path = match proc_macro_crate::crate_name("sawe").unwrap() {
        proc_macro_crate::FoundCrate::Itself => "crate".to_string(),
        proc_macro_crate::FoundCrate::Name(name) => format!("::{name}"),
    };

    let root_name = root.file_name().unwrap_or_default().to_string_lossy().to_string();
    let mut generated = String::from(crate_path);

    generated.push_str("::fsread::Directory{ entries: vec![");

    scan(&root, &root, &root_name, show_root, &profile, &mut generated);
    generated.push_str("]}");

    generated.parse().expect("embed_dir!: failed to generate Rust code")
}

fn compress_file(data: &[u8], profile: &str) -> (Vec<u8>, &'static str) {
    let result = match profile {
        "None" => return (data.to_vec(), "None"),
        "Fast" => (lz4_flex::compress_prepend_size(data), "Lz4"),
        "Balanced" => (
            zstd::bulk::compress(data, 3).expect("zstd compression failed"),
            "Zstd",
        ),
        "Best" => (
            zstd::bulk::compress(data, 15).expect("zstd compression failed"),
            "Zstd",
        ),
        "Maximum" => {
            let mut encoder = brotli::CompressorWriter::new(Vec::new(), 4096, 11, 22);
            encoder.write_all(data).expect("brotli compression failed");
            (encoder.into_inner(), "Brotli")
        }
        other => panic!(
            "unknown compression profile `{other}`; use None, Fast, Balanced, Best, or Maximum"
        ),
    };

    if result.0.len() < data.len() {
        result
    } else {
        (data.to_vec(), "None")
    }
}

fn scan(
    root: &Path,
    current: &Path,
    root_name: &str,
    show_root: bool,
    profile: &str,
    output: &mut String,
) {

    let crate_path = match proc_macro_crate::crate_name("sawe").unwrap() {
        proc_macro_crate::FoundCrate::Itself => "crate".to_string(),
        proc_macro_crate::FoundCrate::Name(name) => format!("::{name}"),
    };

    let mut children: Vec<PathBuf> = fs::read_dir(current)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", current.display()))
        .map(|entry| entry.unwrap().path())
        .collect();

    children.sort();

    for path in children {
        let relative = path.strip_prefix(root).expect("path outside embedding root");
        let relative = if show_root {
            Path::new(root_name).join(relative)
        } else {
            relative.to_path_buf()
        };

        if path.is_dir() {
            output.push_str(&format!(
                "{crate_path}::fsread::Entry::embedded_dir(::std::path::PathBuf::from({:?})),",
                relative.to_string_lossy()
            ));
            scan(root, &path, root_name, show_root, profile, output);
        } else if path.is_file() {
            let original = fs::read(&path)
                .unwrap_or_else(|e| panic!("cannot embed {}: {e}", path.display()));
            let (stored, codec) = compress_file(&original, profile);
            let bytes = stored.iter().map(u8::to_string).collect::<Vec<_>>().join(",");

            output.push_str(&format!(
                "{crate_path}::fsread::Entry::embedded_file(\
                 ::std::path::PathBuf::from({:?}), vec![{}], \
                 {crate_path}::fsread::Codec::{}),",
                relative.to_string_lossy(),
                bytes,
                codec
            ));
        }
    }
}
