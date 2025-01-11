// src/fetch/download.rs
//! Defines download functions

use std::error::Error;
use std::io::{Write, Read};
use std::fs::{self, File};
use std::path::Path;
use crate::package::Package;
use crate::die;
use crate::utils::fail::Fail;
use indicatif::{ProgressBar, ProgressStyle};

pub const BAR: &str = "{msg:.red} [{elapsed_precise}] [{wide_bar:.red/black}] {bytes}/{total_bytes} ({eta})";

pub fn download(package: &Package, force: bool) {
    download_tarball(package, force);
    download_extra(package, force);
}

pub fn download_extra(package: &Package, force: bool) {
    for source in package.data.extra.iter() {
        let file_name = source.url.rsplit_once('/').map(|(_, name)| name.to_string()).fail("Invalid url");
        let out = format!("/usr/ports/{}/.sources/{}", package.relpath, file_name);

        if let Err(e) = download_url(&source.url, &out, force) {
            if !e.to_string().contains("Exists: ") {
                die!("Failed to get extra url '{}': {}", source.url, e)
            }
        }
    }
}

pub fn download_url(url: &str, out: &str, force: bool) -> Result<String, Box<dyn Error>> {
    let out = if out == "url" { url } else { out };
    let file_name = out.rsplit_once('/').map(|(_, name)| name.to_string()).fail("Invalid url");
    let file_path = Path::new(&out);

    if file_path.exists() && !force {
        let erm = format!("Exists: {:?}", file_path);
        return Err(erm.into())
    }

    let r = ureq::get(url).call().unwrap_or_else(|e| die!("Failed to download url '{}': {}", url, e));

    if r.status() != 200 {
        return Err(format!("HTTP Status: {}", r.status()).into());
    }

    let length = r.header("Content-Length").and_then(|len| len.parse().ok()).unwrap_or(8192);
    let bar = ProgressBar::new(length);

    bar.set_message(file_name.clone());
    bar.set_style(
        ProgressStyle::with_template(BAR)
            .unwrap()
            .progress_chars("=>-")
    );
    
    // bar.set_length(length);

    let mut f = File::create(file_path)?;
    match r.header("Content-Type") {
        Some(ct) if ct.starts_with("text/") => {
            let text = r.into_string()?;
            f.write_all(text.as_bytes())?;
        }
        _ => {
            let mut reader = bar.wrap_read(r.into_reader());
            let mut buffer = vec![0; 8192];
            let mut downloaded = 0;

            loop {
                let bytes_read = reader.read(&mut buffer)?;
                if bytes_read == 0 { break }

                f.write_all(&buffer[..bytes_read])?;
                downloaded += bytes_read as u64;

                bar.set_position(downloaded);

                if length < downloaded {
                    bar.set_length(downloaded)
                }
            }
        }
    }

    bar.set_position(length);
    bar.finish_with_message("Done");

    Ok(file_name.to_string())
}

pub fn normalize_tarball(package: &Package, tarball: &str) -> String {
    let ext = tarball.rsplit_once(".t")
        .map(|(_, ext)| format!(".t{}", ext))
        .fail("Unsupported tarball format");

    let to = match ext.as_str() {
        ".tar.bz2"  | ".tbz" | ".tb2" | ".tbz2" | ".tz2" => format!("{}.tar.bz2",  package),
        ".tar.gz"   | ".tgz" | ".taz"                    => format!("{}.tar.gz",   package),
        ".tar.lz"                                        => format!("{}.tar.lz",   package),
        ".tar.lzma" | ".tlz"                             => format!("{}.tar.lzma", package),
        ".tar.lzo"                                       => format!("{}.tar.lzo",  package),
        ".tar.xz"   | ".txz"                             => format!("{}.tar.xz",   package),
        ".tar.zst"  | ".tzst"                            => format!("{}.tar.zst",  package),
        _ => die!("Unsupported tarball extension: {}", ext),
    };

    to.to_string()
}

fn download_tarball(package: &Package, force: bool) {
    let url = package.data.source.url.clone();
    if url.is_empty() { return }

    let file_name = url.split('/').last().expect("Invalid url");
    let file_name = normalize_tarball(package, file_name);

    let srcpath = format!("/usr/ports/{}/.sources/", package.relpath);
    fs::create_dir_all(&srcpath).ufail("Failed to create source path");
    let out = format!("{}/{}", &srcpath, &file_name);

    if let Err(e) = download_url(&url, &out, force) {
        if !e.to_string().contains("Exists: ") {
            die!("Failed to download tarball for '{}': {}", package, e)
        }
    }
}
