use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedSources {
    pub sources: Vec<Source>,
    pub tracks: Vec<Track>,
    pub t: u32,
    pub server: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub file: String,
    #[serde(rename = "type")]
    pub source_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    pub file: String,
    pub label: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

pub fn handle_embed(embed_url: &str, referrer: &str) -> EmbedSources {
    let output = Command::new("node")
        .arg("rabbit.js")
        .arg(format!("--embed-url={}", embed_url))
        .arg(format!("--referrer={}", referrer))
        .output()
        .expect("Failed to execute command");

    let parsed_output = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let embed_json: EmbedSources =
        serde_json::from_str(&parsed_output).expect("Failed to parse JSON");

    embed_json
}
