use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use deno_core::{Extension, JsRuntime, RuntimeOptions};
use sha2::{Digest, Sha256};
use std::env;
use std::io::Cursor;

const DOWNLOAD_DEVTOOLS_URL: &'static str =
    "https://github.com/Nevermore-FMS/devtools-builder/releases/download/v1.0.0/release-bundle.zip";

const DEVTOOLS_SHA256: &'static str =
    "c0846daaaa824e648fe03c1beb5169fae224758faf07e4501c31e8624d5d81ce";

fn main() -> anyhow::Result<()> {
    build_types()?;
    build_snapshot();
    build_devtools()?;
    Ok(())
}

fn build_types() -> std::io::Result<()> {
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let extensions = vec![
        deno_console::get_declaration(),
        deno_url::get_declaration(),
        deno_web::get_declaration(),
        deno_fetch::get_declaration(),
        deno_websocket::get_declaration(),
        deno_crypto::get_declaration(),
        deno_broadcast_channel::get_declaration(),
        deno_net::get_declaration(),
        out_dir
            .join("runtime")
            .join("ts")
            .join("shared-globals.d.ts"),
        out_dir.join("runtime").join("ts").join("nevermore.d.ts"),
        out_dir.join("runtime").join("ts").join("database.d.ts"),
        out_dir.join("runtime").join("ts").join("pubsub.d.ts"),
        out_dir.join("runtime").join("ts").join("network.d.ts"),
    ];

    let mut final_string = String::new();

    for extension in extensions {
        let contents =
            std::fs::read_to_string(extension).expect("Something went wrong reading the file");

        final_string.push_str(&contents);
    }

    let mut file = File::create(out_dir.join("types").join("generated.d.ts"))?;

    file.write_all(final_string.as_bytes())?;
    Ok(())
}

fn build_devtools() -> anyhow::Result<()> {
    if Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("gen")
        .join("devtools")
        .exists()
    {
        return Ok(());
    }
    clean_devtools();
    let result = download_and_extract_devtools();
    if result.is_err() {
        clean_devtools();
        result?;
    }

    Ok(())
}

fn download_and_extract_devtools() -> anyhow::Result<()> {
    use zip::ZipArchive;

    let compressed_file = try_download(DOWNLOAD_DEVTOOLS_URL)?;
    let mut archive = ZipArchive::new(compressed_file)?;

    archive.extract(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("gen")
            .join("devtools"),
    )?;

    Ok(())
}

fn clean_devtools() {
    std::fs::remove_dir_all(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("gen")
            .join("devtools"),
    )
    .ok();
}

// Zip Downloading inspired by libsodium

fn try_download(url: &str) -> anyhow::Result<Cursor<Vec<u8>>> {
    // Send GET request to download
    let response = reqwest::blocking::get(url)?;

    // Only accept 2xx status codes
    if response.status().is_server_error() || response.status().is_client_error() {
        return Err(anyhow::anyhow!(
            "Download error: HTTP {}",
            response.status()
        ));
    }
    let buffer = response.bytes()?.to_vec();

    // Check the SHA-256 hash of the downloaded file is as expected
    let hash = Sha256::digest(&buffer);
    if &format!("{:x}", hash) != DEVTOOLS_SHA256 {
        return Err(anyhow::anyhow!(
            "Downloaded devtools file failed hash check."
        ));
    }
    Ok(Cursor::new(buffer))
}

fn build_snapshot() {
    // Skip building from docs.rs.
    if env::var_os("DOCS_RS").is_some() {
        return;
    }

    // To debug snapshot issues uncomment:
    // op_fetch_asset::trace_serializer();

    println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());
    println!("cargo:rustc-env=PROFILE={}", env::var("PROFILE").unwrap());
    let out = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("src")
        .join("worker")
        .join("v8_snapshots");

    // Main snapshot
    let runtime_snapshot_path = out.join("SNAPSHOT.bin");

    let js_files = get_js_files("runtime/js");
    create_runtime_snapshot(&runtime_snapshot_path, js_files);
}

fn get_js_files(d: &str) -> Vec<PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut js_files = std::fs::read_dir(d)
        .unwrap()
        .map(|dir_entry| {
            let file = dir_entry.unwrap();
            manifest_dir.join(file.path())
        })
        .filter(|path| path.extension().unwrap_or_default() == "js")
        .collect::<Vec<PathBuf>>();
    js_files.sort();
    js_files
}

fn create_snapshot(mut js_runtime: JsRuntime, snapshot_path: &Path, files: Vec<PathBuf>) {
    // TODO(nayeemrmn): https://github.com/rust-lang/cargo/issues/3946 to get the
    // workspace root.
    let display_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    for file in files {
        println!("cargo:rerun-if-changed={}", file.display());
        let display_path = file.strip_prefix(display_root).unwrap();
        let display_path_str = display_path.display().to_string();
        js_runtime
            .execute_script(
                &("deno:".to_string() + &display_path_str.replace('\\', "/")),
                &std::fs::read_to_string(&file).unwrap(),
            )
            .unwrap();
    }

    let snapshot = js_runtime.snapshot();
    let snapshot_slice: &[u8] = &*snapshot;
    println!("Snapshot size: {}", snapshot_slice.len());
    std::fs::write(&snapshot_path, snapshot_slice).unwrap();
    println!("Snapshot written to: {} ", snapshot_path.display());
}

fn create_runtime_snapshot(snapshot_path: &Path, files: Vec<PathBuf>) {
    let extensions: Vec<Extension> = vec![
        deno_webidl::init(),
        deno_console::init(),
        deno_url::init(),
        deno_web::init(Default::default(), Default::default()),
        deno_fetch::init::<deno_fetch::NoFetchPermissions>("".to_owned(), None, None),
        deno_net::init::<deno_net::NoNetPermissions>(false),
        deno_websocket::init::<deno_websocket::NoWebSocketPermissions>("".to_owned(), None),
        deno_crypto::init(None),
        deno_timers::init::<deno_timers::NoTimersPermission>(),
        deno_broadcast_channel::init(
            deno_broadcast_channel::InMemoryBroadcastChannel::default(),
            false, // No --unstable.
        ),
    ];

    let js_runtime = JsRuntime::new(RuntimeOptions {
        will_snapshot: true,
        extensions,
        ..Default::default()
    });
    create_snapshot(js_runtime, snapshot_path, files);
}
