/// Search interface to extensions.gnome.org
use crate::gnome;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShellVersionMap {
    pub pk: i64,
    pub version: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Extension {
    pub uuid: String,
    pub name: String,
    pub creator: String,
    pub creator_url: String,
    pub pk: i32,
    pub description: String,
    pub link: String,
    pub icon: String,
    pub shell_version_map: HashMap<String, ShellVersionMap>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Extensions {
    pub extensions: Vec<Extension>,
}

/// Search extensions.gnome.org by keyword
pub fn search(keywords: String) -> Result<Extensions, Box<dyn std::error::Error>> {
    let url = format!(
        "https://extensions.gnome.org/extension-query/?search={}&sort=popularity&shell_version={}&page={}",
        keywords,
        gnome::get_shell_version()?,
        1
    );

    // download the json
    let response = reqwest::blocking::get(url)?;

    let json = response.text()?;

    let p: Extensions = serde_json::from_str(json.as_str())?;

    Ok(p)
}
