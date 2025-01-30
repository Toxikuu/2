// src/fetch/download.rs
//! Defines download functions

use anyhow::{bail, Context};
use crate::package::Package;
use crate::utils::fail::{fail, ufail, Fail};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::Path;

/// # Description
/// The format for the download bar
const BAR: &str = "{msg:.red} [{elapsed_precise}] [{wide_bar:.red/black}] {bytes}/{total_bytes} ({eta})";

/// # Description
/// Very high level download function for package
///
/// Downloads the tarball and any extra sources
pub fn download(package: &Package, force: bool) {
    download_tarball(package, force);
    download_extra(package, force);
}

/// # Description
/// Downloads any extra sources for a package, calculating the file name from the urls. If none are
/// provided, doesn't download anything.
///
/// Affected by force
/// 
/// **Fail conditions:**
/// - source path could not be created
/// - url was invalid
/// - ``download_url()`` returns an error other than Exists
///
/// Saves the downloaded sources to ``/usr/ports/<repo>/<package>/.sources/<name>``
pub fn download_extra(package: &Package, force: bool) {
    package.data.extra.iter().for_each(|source| {
        let file_name = source.url.rsplit_once('/').map(|(_, name)| name.to_string()).fail(&format!("Invalid extra url: '{}'", source.url));
        let out = format!("/usr/ports/{}/.sources/{}", package.relpath, file_name);

        if let Err(e) = download_url(&source.url, &out, force) {
            if !e.to_string().contains("Exists: ") {
                fail!("Failed to get extra url '{}'", source.url);
            }
        }
    });
}

/// # Description
/// Lower level download function
///
/// Downloads a specific url to an output destination; that output destination must be manually
/// specified, and for 2, is usually in .sources
///
/// **Error conditions:**
/// - the output path exists and force is not passed. Will overwrite if force is passed.
/// - the http status is not 200
/// - the file path cannot be created (unlikely)
/// - random buffer-related rw failures (unlikely)
pub fn download_url(url: &str, out: &str, force: bool) -> anyhow::Result<String> {
    let file_name = out.rsplit_once('/').map(|(_, name)| name.to_string()).ufail("Invalid output path");
    let file_path = Path::new(&out);

    if file_path.exists() && !force {
        bail!("Exists: {:?}", file_path);
    }

    let r = ureq::get(url).call().fail(&format!("Failed to download url '{url}'"));

    if r.status() != 200 {
        bail!("HTTP Status: {}", r.status());
    }

    let length = r.header("Content-Length").and_then(|len| len.parse().ok()).unwrap_or(8192);
    let bar = ProgressBar::new(length);

    bar.set_message(file_name.clone());
    bar.set_style(
        ProgressStyle::with_template(BAR)
            .ufail("Invalid template for indicatif bar")
            .progress_chars("=>-")
    );

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
                    bar.set_length(downloaded);
                }
            }
        }
    }

    bar.set_position(length);
    bar.finish_with_message("Done");

    Ok(file_name)
}

/// # Description
/// Normalizes tarball extensions to their long form. This is used to calculate the tarball file
/// name in ``download_tarball()``. It's also used to calculate the tarball name for hash checks.
///
/// **Fail conditions:**
/// - an unsupported tarball extension is passed
///
/// **Examples:**
/// - ``whois-1.0.0.tbz`` -> ``whois=1.0.0.tar.bz2``
/// - ``tree_2.2.1.taz`` -> ``tree=2.2.1.tar.gz``
/// - ``tar_src.stupid_tarball_name=1.35.0.tar.zst`` -> ``tar=1.35.tar.zst``
pub fn normalize_tarball(package: &Package, tarball: &str) -> String {
    let ext = tarball.rsplit_once(".t")
        .map(|(_, ext)| format!(".t{ext}"))
        .fail("Unsupported tarball format");

    // if failures occur, i may use .tar.xz as a generic fallback, even if it's inaccurate
    let to = match ext.as_str() {
        ".tar.bz2"  | ".tbz" | ".tb2" | ".tbz2" | ".tz2" => format!("{package}.tar.bz2" ),
        ".tar.gz"   | ".tgz" | ".taz"                    => format!("{package}.tar.gz"  ),
        ".tar.lz"                                        => format!("{package}.tar.lz"  ),
        ".tar.lzma" | ".tlz"                             => format!("{package}.tar.lzma"),
        ".tar.lzo"                                       => format!("{package}.tar.lzo" ),
        ".tar.xz"   | ".txz"                             => format!("{package}.tar.xz"  ),
        ".tar.zst"  | ".tzst"                            => format!("{package}.tar.zst" ),
        _ => ufail!("Unsupported tarball extension: {}", ext),
    };

    to
}

/// # Description
/// Downloads a package's tarball. If a source url is not provided, returns early without
/// downloading anything.
///
/// Affected by force
/// 
/// **Fail conditions:**
/// - source path could not be created
/// - url was invalid
/// - ``download_url()`` returns an error other than Exists
///
/// Saves the downloaded sources to ``/usr/ports/<repo>/<package>/.sources/<name>``
fn download_tarball(package: &Package, force: bool) {
    let url = package.data.source.url.clone();
    if url.is_empty() { return }

    let file_name = url.split('/').next_back().context("Likely the repo's maintainer's fault").fail("Invalid url");
    let file_name = normalize_tarball(package, file_name);

    let srcpath = format!("/usr/ports/{}/.sources/", package.relpath);
    fs::create_dir_all(&srcpath).ufail("Failed to create source path");
    let out = format!("{}/{}", &srcpath, &file_name);

    if let Err(e) = download_url(&url, &out, force) {
        if !e.to_string().contains("Exists: ") {
            fail!("Failed to download tarball for '{}'", package);
        }
    }
}
