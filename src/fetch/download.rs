// src/fetch/download.rs
//
// defines download functions

use std::error::Error;
use std::io::{self, Write};
use std::fs::{self, File};
use std::path::Path;
use crate::package::Package;
use crate::die;
use indicatif::{ProgressBar, ProgressStyle};

pub const BAR: &str = "{msg:.red} [{elapsed_precise}] [{wide_bar:.red/black}] {bytes}/{total_bytes} ({eta})";

pub fn download(package: &Package, force: bool) {
    download_tarball(package, force);
    download_extra(package, force);
}

pub fn download_extra(package: &Package, force: bool) {
    let relpath = format!("{}/{}", package.repo, package.name);
    for source in package.data.extra.iter() {
        let file_name = source.url.rsplit_once('/').map(|(_, name)| name.to_string()).unwrap();
        let out = format!("/sources/{}/{}", relpath, file_name);

        if let Err(e) = download_url(&source.url, &out, force) {
            if !e.to_string().contains("Exists: ") {
                die!("Failed to get extra url '{}': {}", source.url, e)
            }
        }
    }
}

pub fn download_url(url: &str, out: &str, force: bool) -> Result<String, Box<dyn Error>> {
    let out = if out == "url" { url } else { out };
    let file_name = out.rsplit_once('/').map(|(_, name)| name.to_string()).unwrap();
    let file_path = Path::new(&out);

    if file_path.exists() && !force {
        let erm = format!("Exists: {:?}", file_path);
        return Err(erm.into())
    }

    let r = ureq::get(url).call().unwrap_or_else(|e| die!("Failed to get '{}': {}", url, e));

    if r.status() != 200 {
        return Err(format!("HTTP Status: {}", r.status()).into());
    }

    let length = r.header("Content-Length").and_then(|len| len.parse().ok());
    let bar = match length {
        Some(len) => ProgressBar::new(len),
        _ => ProgressBar::new_spinner(),
    };

    bar.set_message(file_name.clone());
    bar.set_style(
        ProgressStyle::with_template(BAR)
            .unwrap()
            .progress_chars("=>-")
    );

    if let Some(len) = length { bar.set_length(len) }

    let mut f = File::create(file_path)?;
    match r.header("Content-Type") {
        Some(ct) if ct.starts_with("text/") => {
            let text = r.into_string()?;
            f.write_all(text.as_bytes())?;
        }
        _ => {
            io::copy(&mut bar.wrap_read(r.into_reader()), &mut f).map(|_| ())?;
        }
    }

    bar.finish_with_message("Done");
    bar.finish_using_style();

    Ok(file_name.to_string())
}

fn normalize_tarball(package: &Package, tarball: &str) -> String {
    let ext = tarball.rsplit_once(".t")
        .map(|(_, ext)| format!(".t{}", ext))
        .unwrap_or_else(|| die!("Unsupported tarball format for '{}'", tarball));

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

// TODO: Add hash checks
fn download_tarball(package: &Package, force: bool) {
    let url = package.data.source.url.clone();
    let file_name = url.split('/').last().expect("Invalid url");
    let file_name = normalize_tarball(package, file_name);

    let relpath = format!("{}/{}", package.repo, package.name);
    let srcpath = format!("/sources/{}", relpath);
    fs::create_dir_all(&srcpath).unwrap();
    let out = format!("{}/{}", &srcpath, &file_name);

    if let Err(e) = download_url(&url, &out, force) {
        if !e.to_string().contains("Exists: ") {
            die!("Failed to download tarball for '{}': {}", package, e)
        }
    }
}
