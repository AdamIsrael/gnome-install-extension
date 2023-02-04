use crate::gnome;
use crate::search;

use std::io::{copy, Cursor};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process;

use dirs::home_dir;
use regex::Regex;
use reqwest;

/// Download a url to a temporary file
// pub fn download(url: String) -> Result<(), Box<dyn std::error::Error>> {
pub fn download(url: String) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // let tmp_dir = Builder::new().prefix("example").tempdir()?;

    let tmp_dir = std::env::temp_dir()
        .join(format!("gnome-extension-tool-{}", process::id()));

    fs::create_dir(tmp_dir.clone())?;

    let response = reqwest::blocking::get(url)?;

    let dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = tmp_dir.join(fname);
        // let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        let mut fh = File::create(fname.clone())?;

        let mut content =  Cursor::new(response.bytes()?);
        copy(&mut content, &mut fh)?;
  
        fname
    };

    Ok(dest)

}


/// Install a GNOME extension from url
pub fn install(url: String) -> Result<(), Box<dyn std::error::Error>> {

    // Parse the url for the pk
    // https://extensions.gnome.org/extension/4679/burn-my-windows/

    let gnome_shell_version = gnome::get_shell_version()?;
    println!("Version: {:?}", gnome_shell_version);
    if 0 < gnome_shell_version {

        let uuid = get_uuid_by_url(url.clone().to_string())?;

        // Search by uuid
        let results = search::search(uuid.to_string());
        match results {
            Ok(extensions) => {
                let extension = extensions.extensions.into_iter().nth(0).unwrap();
                println!("extension uuid: {}", extension.uuid);
                let pk = extension.shell_version_map.get(gnome_shell_version.to_string().as_str()).unwrap().pk;
                // extension.
                let download_url = format!(
                    "https://extensions.gnome.org/download-extension/{}.shell-extension.zip?version_tag={}",
                    extension.uuid,
                    pk
                );

                match download(download_url) {
                    Ok(dest) => {
                        println!("Got zip: {:?}", dest);

                        // `gnome-extension install` doesn't seem to work for me.
                        // Let's bypass it and do it manually.
                        // Unzip the file to ~/.local/share/gnome-shell/extensions/{uuid}, i.e.
                        // ~/.local/share/gnome-shell/extensions/gsconnect@andyholmes.github.io
                        install_zip(dest, uuid);

                        // enable_extension(extension.uuid);

                    },
                    Err(error) => { panic!("Error: {}", error)},
                }
            },
            Err(error) => { panic!("Error searching extensions.gnome.org: {:?}", error) }
        }
    }

    Ok(())
}

/// Get the extension's UUID from it's homepage
fn get_uuid_by_url(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let html = response.text()?;

    let re = Regex::new(
        "data-uuid=\"([^\"]+)"
    ).unwrap();
    let caps = re.captures(html.as_str()).ok_or("Couldn't find UUID")?;
    let uuid = caps.get(1).unwrap().as_str().to_string();

    Ok(uuid)
}

/// Install an extension via zip file
fn install_zip(path: PathBuf, uuid: String) -> Result<(), Box<dyn std::error::Error>> {

    // unzip the file to ~/.local/share/gnome-shell/extensions/<uuid>
    let file = fs::File::open(path).unwrap();

    println!("File: {:?}", file);
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        // Build the path to 
        let outpath = match file.enclosed_name() {
            Some(path) => Path::new(&home_dir().unwrap())
                .join(Path::new(".local/share/gnome-shell/extensions"))
                .join(uuid.to_owned())
                .join(path.to_owned()),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            // println!(
            //     "File {} extracted to \"{}\" ({} bytes)",
            //     i,
            //     outpath.display(),
            //     file.size()
            // );
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

// fn enable_extension(uuid: String) -> Result<(), Box<dyn std::error::Error>> {

//     let output = if cfg!(target_os = "linux") {
//         Command::new("gnome-extensions")
//                 .args(["enable", uuid.as_str()])
//                 .output()
//                 .expect("failed to execute process")
//     } else {
//         panic!("Only Linux is supported.")
//     };

//     let tmp = String::from_utf8(output.stdout).unwrap();
//     println!("Output: {}", tmp);

//     Ok(())
// }