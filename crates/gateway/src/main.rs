use std::env;
use std::path::Path;

use tiktok_direct_engine::{download_media, MediaKind, TikTokExtractor};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let command = args.next().ok_or_else(usage)?;
    let url = args.next().ok_or_else(usage)?;

    match command.as_str() {
        "extract" => {
            let metadata = TikTokExtractor::new()
                .extract(&url)
                .map_err(|err| err.to_string())?;
            println!(
                "{}",
                serde_json::to_string(&metadata).map_err(|err| err.to_string())?
            );
        }
        "download" => {
            let kind = args.next().ok_or_else(usage)?;
            let output = args.next();
            let kind = MediaKind::parse(&kind).map_err(|err| err.to_string())?;
            let metadata = TikTokExtractor::new()
                .extract(&url)
                .map_err(|err| err.to_string())?;
            let output = output.as_deref().map(Path::new);
            let path = download_media(&metadata, kind, output).map_err(|err| err.to_string())?;
            println!("{}", path.to_string_lossy());
        }
        _ => return Err(usage()),
    }

    Ok(())
}

fn usage() -> String {
    "usage: tiktok-direct-gateway <extract URL | download URL KIND [OUTPUT]>".to_string()
}
