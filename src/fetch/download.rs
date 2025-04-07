// src/fetch/download.rs
//! Defines download functions

use std::{
    fs::File,
    io::{
        Read,
        Write,
    },
    path::Path,
};

use anyhow::{
    Context,
    Result,
    bail,
};
use indicatif::{
    ProgressBar,
    ProgressStyle,
};
use ureq::{
    Error as UE,
    http::header::CONTENT_LENGTH,
    // http::header::{CONTENT_LENGTH, CONTENT_TYPE},
};

use crate::{
    package::Package,
    utils::{
        comms::vpr,
        fail::{
            BoolFail,
            Fail,
        },
    },
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
/// Downloads any extra sources for a package, calculating the file name from
/// the urls. If none are provided, doesn't download anything.
///
/// Affected by force
///
/// **Fail conditions:**
/// - source path could not be created
/// - url was invalid
/// - ``download_url()`` returns an error other than Exists
///
/// Saves the downloaded sources to ``/var/ports/<repo>/<package>/.sources/<name>``
pub fn download_extra(package: &Package, force: bool, sty: &ProgressStyle) -> bool {
    let mut dlct = 0;
    package.extra.iter().for_each(|source| {
        let url = &source.url;
        let file_name = url
            .rsplit_once('/')
            .map(|(_, name)| name.to_string())
            .efail(|| format!("Invalid extra url '{url}' for '{package}'"));
        let out = package.data.port_dir.join(".sources").join(&file_name);

        if let Err(e) = download_url(&source.url, &out, force, sty) {
            e.to_string()
                .contains("Exists: ")
                .or_efail(|| format!("Failed to get extra url '{url}' for '{package}'"));
        }
        dlct += 1;
    });
    dlct >= 1
}

/// # Description
/// Lower level download function
///
/// Downloads a specific url to an output destination; that output destination
/// must be manually specified, and for 2, is usually in .sources
///
/// **Error conditions:**
/// - the output path exists and force is not passed. Will overwrite if force is passed.
/// - the http status is not 200
/// - the file path cannot be created (unlikely)
/// - random buffer-related rw failures (unlikely)
pub fn download_url(url: &str, out: &Path, force: bool, sty: &ProgressStyle) -> Result<String> {
    let file_name = out
        .file_name()
        .context("Failed to get filename")?
        .to_string_lossy()
        .to_string();
    let file_path = Path::new(&out);

    if file_path.exists() && !force {
        bail!("Exists: {:?}", file_path);
    }

    vpr!("Downloading '{url}'...");
    let r = match ureq::get(url).call() {
        | Ok(r) => r,
        | Err(UE::StatusCode(code)) => bail!("Received status code '{code}'"),
        | Err(UE::HostNotFound) => bail!("Failed to resolve hostname"),
        | Err(_) => bail!("An unexpected error occured"),
    };
    vpr!("Response:\n{r:#?}");

    let length: u64 = r
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or(8192);

    let pb = ProgressBar::new(length);
    pb.set_style(sty.clone());
    pb.set_length(length);
    pb.set_prefix("󰇚 ");
    pb.set_message(file_name.clone());

    let mut f = File::create(file_path)?;

    let body = r.into_body();
    let reader = body.into_reader();

    let mut downloaded = 0;
    let mut reader = pb.wrap_read(reader);
    let mut buffer = vec![0; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        f.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;

        pb.set_position(downloaded);

        if length < downloaded {
            pb.set_length(downloaded);
        }
    }

    pb.set_position(length);
    pb.set_prefix("󰄹 ");
    pb.finish_with_message(file_name.clone());

    Ok(file_name)
}

/// # Description
/// Normalizes tarball extensions to their long form. This is used to calculate
/// the tarball file name in ``download_tarball()``. It's also used to calculate
/// the tarball name for hash checks.
///
/// **Fail conditions:**
/// - an unsupported tarball extension is passed
///
/// **Examples:**
/// - ``whois-1.0.0.tbz`` -> ``whois=1.0.0.tar.bz2``
/// - ``tree_2.2.1.taz`` -> ``tree=2.2.1.tar.gz``
/// - ``tar_src.stupid_tarball_name=1.35.0.tar.zst`` -> ``tar=1.35.tar.zst``
pub fn normalize_tarball(package: &Package, tarball: &str) -> String {
    let ext = tarball
        .rsplit_once(".t")
        .map(|(_, ext)| format!(".t{ext}"))
        .efail(|| format!("[UNREACHABLE] Unsupported tarball format for tarball '{tarball}'"));

    let to = match ext.as_str() {
        | ".tar.bz2" | ".tbz" | ".tb2" | ".tbz2" | ".tz2" => format!("{package}.tar.bz2"),
        | ".tar.gz" | ".tgz" | ".taz" => format!("{package}.tar.gz"),
        | ".tar.lz" => format!("{package}.tar.lz"),
        | ".tar.lzma" | ".tlz" => format!("{package}.tar.lzma"),
        | ".tar.lzo" => format!("{package}.tar.lzo"),
        | ".tar.xz" | ".txz" => format!("{package}.tar.xz"),
        | ".tar.zst" | ".tzst" => format!("{package}.tar.zst"),
        | _ => unreachable!(
            "Unsupported tarball extension '{ext}' for tarball '{tarball}'.\nYour ass should not be seeing this error.\nWtf did you do?"
        ),
    };

    to
}

/// # Description
/// Downloads a package's tarball. If a source url is not provided, returns
/// early without downloading anything.
///
/// Affected by force
///
/// **Fail conditions:**
/// - source path could not be created
/// - url was invalid
/// - ``download_url()`` returns an error other than Exists
///
/// Saves the downloaded sources to ``/var/ports/<repo>/<package>/.sources/<name>``
fn download_tarball(package: &Package, force: bool, sty: &ProgressStyle) -> bool {
    let url = package.source.url.clone();
    if url.is_empty() {
        return false;
    }

    let file_name = url
        .split('/')
        .next_back()
        .context("Likely the repo's maintainer's fault")
        .efail(|| format!("Invalid url '{url}' for '{package}'"));
    let file_name = normalize_tarball(package, file_name);

    let srcpath = package.data.port_dir.join(".sources");
    let out = srcpath.join(file_name);

    vpr!("Downloading tarball...");
    if let Err(e) = download_url(&url, &out, force, sty) {
        e.to_string()
            .contains("Exists: ")
            .or_efail(|| format!("Failed to download tarball from '{url}' for '{package}'"));
        return false;
    }
    true
}
