// src/fetch/download.rs
//! Defines download functions

use anyhow::{bail, Context};
use crate::{
    package::Package,
    utils::fail::{fail, ufail, Fail},
    comms::log::vpr,
};
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};
use ureq::{
    Error as UE,
    http::header::CONTENT_LENGTH,
    // http::header::{CONTENT_LENGTH, CONTENT_TYPE},
};

pub enum DownloadStatus {
    Nothing,
    Tarball,
    Extra,
    Both,
}

/// # Description
/// Very high level download function for package
///
/// Downloads the tarball and any extra sources
pub fn download(package: &Package, force: bool, sty: &ProgressStyle) -> DownloadStatus {
    let tb = download_tarball(package, force, sty);
    let ex = download_extra(package, force, sty);

    if ex && tb {
        DownloadStatus::Both
    } else if ex {
        DownloadStatus::Extra
    } else if tb {
        DownloadStatus::Tarball
    } else {
        DownloadStatus::Nothing
    }
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
pub fn download_extra(package: &Package, force: bool, sty: &ProgressStyle) -> bool {
    let mut dlct = 0;
    package.data.extra.iter().for_each(|source| {
        let file_name = source.url.rsplit_once('/').map(|(_, name)| name.to_string()).fail(&format!("Invalid extra url: '{}'", source.url));
        let out = format!("/usr/ports/{}/.sources/{}", package.relpath, file_name);

        if let Err(e) = download_url(&source.url, &out, force, sty) {
            if !e.to_string().contains("Exists: ") {
                fail!("Failed to get extra url '{}'", source.url);
            }
        }
        dlct += 1;
    });
    dlct >= 1
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
pub fn download_url(url: &str, out: &str, force: bool, sty: &ProgressStyle) -> anyhow::Result<String> {
    let file_name = out.rsplit_once('/').map(|(_, name)| name.to_string()).ufail("Invalid output path");
    let file_path = Path::new(&out);

    if file_path.exists() && !force {
        bail!("Exists: {:?}", file_path);
    }

    vpr!("Downloading '{url}'...");
    let r = match ureq::get(url).call() {
        Ok(r) => r,
        Err(UE::StatusCode(code)) => bail!("Received status code '{code}'"),
        Err(UE::HostNotFound) => bail!("Failed to resolve hostname"),
        Err(_) => bail!("An unexpected error occured")
    };
    vpr!("Response:\n{r:#?}");

    let length: u64 = r.headers()
        .get(CONTENT_LENGTH)
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or(8192);

    let pb = ProgressBar::new(length);
    pb.set_style(sty.clone());
    pb.set_length(length);
    pb.set_prefix("󱑤 ");
    pb.set_message(file_name.clone());

    let mut f = File::create(file_path)?;

    // let is_text = r.headers()
    //     .get(CONTENT_TYPE)
    //     .and_then(|ct| ct.to_str().ok())
    //     .is_some_and(|s| s.starts_with("text/"));

    let body = r.into_body();
    let reader = body.into_reader();

    let mut downloaded = 0;
    let mut reader = pb.wrap_read(reader);
    let mut buffer = vec![0; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 { break }

        f.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;

        pb.set_position(downloaded);

        if length < downloaded {
            pb.set_length(downloaded);
        }
    }

    pb.set_position(length);
    pb.set_prefix("󰗠 ");
    pb.finish_with_message(file_name.clone());

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
fn download_tarball(package: &Package, force: bool, sty: &ProgressStyle) -> bool {
    let url = package.data.source.url.clone();
    if url.is_empty() { return false }

    let file_name = url.split('/').next_back().context("Likely the repo's maintainer's fault").fail("Invalid url");
    let file_name = normalize_tarball(package, file_name);

    let srcpath = format!("/usr/ports/{}/.sources/", package.relpath);
    fs::create_dir_all(&srcpath).ufail("Failed to create source path");
    let out = format!("{}/{}", &srcpath, &file_name);

    vpr!("Downloading tarball...");
    if let Err(e) = download_url(&url, &out, force, sty) {
        if !e.to_string().contains("Exists: ") {
            fail!("Failed to download tarball for '{package}': {e}");
        }
        return false
    }
    true
}
