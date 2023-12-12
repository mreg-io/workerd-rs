use std::env;
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use tar::Archive;

fn main() {
    // Disable build script rerun
    println!("cargo:rerun-if-changed=build.rs");

    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let root = Path::new(out_dir.as_str());
    let (package, path) = get_package_and_path(root);

    // if workerd doesn't exist, download it from npm registry
    if !path.is_file() {
        download_workerd(package, version.as_str(), root).unwrap();
    }
    println!("cargo:rustc-env=WORKERD_PATH={}", path.to_str().unwrap());
}

fn get_package_and_path(root: &Path) -> (&'static str, Box<Path>) {
    let packages = HashMap::from([
        ("aarch64-apple-darwin", "@cloudflare/workerd-darwin-arm64"),
        ("x86_64-apple-darwin", "@cloudflare/workerd-darwin-64"),
        ("aarch64-unknown-linux-gnu", "@cloudflare/workerd-linux-arm64"),
        ("x86_64-unknown-linux-gnu", "@cloudflare/workerd-linux-64"),
        ("x86_64-pc-windows-msvc", "@cloudflare/workerd-windows-64"),
    ]);

    let target = env::var("TARGET").unwrap();
    let package = *packages.get(target.as_str()).unwrap();

    // Build workerd path for specific build and platform
    let mut path = PathBuf::from(root);
    path.push("package");
    path.push("bin");
    if package == "@cloudflare/workerd-windows-64" {
        path.push("workerd.exe");
    } else {
        path.push("workerd");
    }

    (package, path.into_boxed_path())
}

fn download_workerd<'a>(package: &str, version: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    let unscoped_package = package.get(package.find('/').unwrap() + 1..).unwrap();
    let url = format!("https://registry.npmjs.org/{}/-/{}-{}.tgz", package, unscoped_package, version);

    let response = reqwest::blocking::get(url)?;
    let gz = GzDecoder::new(response);
    let mut archive = Archive::new(gz);
    archive.set_preserve_permissions(true);
    archive.unpack(path)?;

    Ok(())
}
