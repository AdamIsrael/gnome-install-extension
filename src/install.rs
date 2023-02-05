use crate::gnome;
use crate::search;
use crate::Args;

use std::fs;
use std::fs::File;
use std::io::{copy, Cursor};
use std::path::{Path, PathBuf};
use std::process;

use dirs::home_dir;
use regex::Regex;

/// Download a url to a temporary file
pub fn download(url: String) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let tmp_dir = std::env::temp_dir().join(format!("gnome-extension-tool-{}", process::id()));

    fs::create_dir(tmp_dir.clone())?;

    let response = reqwest::blocking::get(url)?;

    let dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        let fname = tmp_dir.join(fname);
        let mut fh = File::create(fname.clone())?;

        let mut content = Cursor::new(response.bytes()?);
        copy(&mut content, &mut fh)?;
        fname
    };
    Ok(dest)
}

/// Install a GNOME extension from url, uuid, or keywords
pub fn install(args: &Args) -> Result<String, Box<dyn std::error::Error>> {
    let gnome_shell_version = gnome::get_shell_version()?;
    if 0 == gnome_shell_version {
        panic!("Couldn't determine GNOME shell version.");
    }
    let uuid: String;
    let mut search: String = args.search.join(" ");

    // If this appears to be a URL, open it and parse out the UUID
    if search.starts_with("http") {
        let uuid = get_uuid_by_url(&search)?;
        search = uuid;
    }

    // Search extensions.gnome.org
    let results = search::search(search.as_str());
    match results {
        Ok(extensions) => {
            let extension = extensions.extensions.into_iter().next().unwrap();
            uuid = extension.uuid;
            let pk = extension
                .shell_version_map
                .get(gnome_shell_version.to_string().as_str())
                .unwrap()
                .pk;

            if !args.dry_run {
                let download_url = format!(
                    "https://extensions.gnome.org/download-extension/{uuid}.shell-extension.zip?version_tag={pk}",
                );

                match download(download_url) {
                    Ok(dest) => {
                        // Unzip the file to ~/.local/share/gnome-shell/extensions/{uuid}, i.e.
                        // ~/.local/share/gnome-shell/extensions/gsconnect@andyholmes.github.io
                        match install_zip(dest, &uuid) {
                            Ok(_ok) => {
                                gnome::enable_extension(&uuid);
                                if !args.quiet {
                                    println!(
                                        "Extension {uuid:?} successfully installed and enabled."
                                    )
                                }
                            }
                            Err(e) => panic!("Unable to install zip file: {e:?}"),
                        }
                    }
                    Err(error) => {
                        panic!("Error: {error}")
                    }
                }
            } else if !args.quiet {
                println!("Skipping installation of {uuid:?}. (dry-run)")
            }
        }
        Err(error) => {
            panic!("Error searching extensions.gnome.org: {error:?}")
        }
    }
    Ok(uuid)
}

/// Get the extension's UUID from it's homepage
fn get_uuid_by_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let uuid = {
        let response = reqwest::blocking::get(url)?;
        let html = response.text()?;

        let re = Regex::new("data-uuid=\"([^\"]+)").unwrap();
        let caps = re.captures(html.as_str()).ok_or("Couldn't find UUID")?;
        caps.get(1).unwrap().as_str().to_owned()
    };

    Ok(uuid)
}

/// Install an extension via zip file
fn install_zip(path: PathBuf, uuid: &str) -> Result<(), Box<dyn std::error::Error>> {
    // unzip the file to ~/.local/share/gnome-shell/extensions/<uuid>
    let file = fs::File::open(path).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        // Build the path to
        let outpath = match file.enclosed_name() {
            Some(path) => Path::new(&home_dir().unwrap())
                .join(Path::new(".local/share/gnome-shell/extensions"))
                .join(uuid)
                .join(path),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            copy(&mut file, &mut outfile).unwrap();
        }
    }

    Ok(())
}
