use anyhow::{Context, Result};
use rust_embed::RustEmbed;

/// Embed the static assets inside of the WebAssembly module
#[derive(RustEmbed)]
#[folder = "./ui/build"]
pub(crate) struct Asset;

/// Retrieve a static asset from disk
pub(crate) fn get_static_asset(asset: &str) -> Result<Vec<u8>> {
    let asset_request = if asset.trim() == "/" || asset.trim().is_empty() {
        "index.html"
    } else {
        asset.trim().trim_start_matches('/')
    };

    Asset::get(asset_request)
        .map(|file| Vec::from(file.data))
        .context("failed to find asset")
}
