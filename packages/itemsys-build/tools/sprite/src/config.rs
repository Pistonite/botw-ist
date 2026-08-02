use std::collections::BTreeMap;
use std::path::PathBuf;

use cu::pre::*;

pub fn load() -> cu::Result<Config> {
    let root = find_root()?;
    let config_str = cu::fs::read_string(root.join("Sprite.yaml"))?;
    let mut config = yaml::parse::<Config>(&config_str)?;
    config.target = root.join("target");
    config.root = root;
    Ok(config)
}

/// Find the package root directory, only works when running from cargo
fn find_root() -> cu::Result<PathBuf> {
    let e = cu::fs::current_exe()?;
    let root_path = e
        .parent() // /target/release
        .and_then(|x| x.parent()) // /target
        .and_then(|x| x.parent()) // /
        .ok_or_else(|| cu::fmterr!("Could not find parent of exe"))?;
    let mut path = root_path.to_path_buf();
    // check
    path.push("Sprite.yaml");
    cu::ensure!(
        path.exists(),
        "could not find Sprite.yaml, make sure you are running through the taskfile or cargo."
    )?;
    path.pop();
    path.normalize()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)]
    pub root: PathBuf,
    #[serde(skip)]
    pub target: PathBuf,
    /// chunk_name -> globs
    pub chunks: BTreeMap<String, Vec<String>>,
    pub render: RenderConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderConfig {
    pub profiles: BTreeMap<String, RenderProfileConfig>,
    pub groups: BTreeMap<String, RenderGroupConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderGroupConfig {
    pub sprites_per_side: u32,
    pub chunks: Vec<RenderChunkConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderProfileConfig {
    pub outer_size: u32,
    pub inner_size: u32,
    pub quality: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderChunkConfig {
    pub chunk: String,
    pub emit: Vec<RenderGroupEmitConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderGroupEmitConfig {
    pub path: String,
    pub profile: String,
}
