use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let extensions = vec![
        deno_console::get_declaration(),
        deno_url::get_declaration(),
        deno_web::get_declaration(),
        deno_fetch::get_declaration(),
        deno_websocket::get_declaration(),
        deno_crypto::get_declaration(),
        deno_broadcast_channel::get_declaration(),
        out_dir.join("ts").join("shared-globals.d.ts")
    ];

    let mut final_string = String::new();

    for extension in extensions {
        let contents = std::fs::read_to_string(extension)
            .expect("Something went wrong reading the file");

        final_string.push_str(&contents);
    }

    let mut file = File::create(out_dir.join("types").join("generated.d.ts"))?;

    file.write_all(final_string.as_bytes())?;
    Ok(())
}