use serde::Serialize;
use std::ffi::OsStr;
use std::path::PathBuf;
use ts_rs::TS;

fn main() {
    let plugin_config = PluginConfig::default();

    let mut plugins: Vec<Plugin> = Vec::new();
    find_vst3_plugins(plugin_config.vst3_path, 0, &mut plugins);
}

pub fn find_plugins(config: Option<PluginConfig>) -> Vec<Plugin> {
    let plugin_config = match config {
        Some(config) => config,
        None => PluginConfig::default(),
    };

    let mut plugins: Vec<Plugin> = Vec::new();
    find_vst3_plugins(plugin_config.vst3_path, 0, &mut plugins);

    plugins
}

fn find_vst3_plugins(base_path: PathBuf, depth: u8, plugins: &mut Vec<Plugin>) {
    for entry in base_path.read_dir().unwrap().flatten() {
        if entry.path().is_dir() && depth <= 5 {
            find_vst3_plugins(entry.path(), depth + 1, plugins);
        } else if entry
            .path()
            .extension()
            .is_some_and(|x| x == OsStr::new("vst3"))
        {
            let path = entry.path().clone();
            let vendor = path
                .ancestors()
                .nth(depth as usize)
                .and_then(|x| x.file_name())
                .and_then(|x| x.to_str())
                .map(|x| x.into());
            plugins.push(Plugin {
                plugin_type: PluginType::Vst3,
                name: entry
                    .file_name()
                    .into_string()
                    .unwrap()
                    .strip_suffix(".vst3")
                    .unwrap()
                    .into(),
                vendor,
                path_to_file: entry.path(),
            })
        }
    }
}



pub const DEFAULT_VST3_PATH: &str = r"C:\Program Files\Common Files\VST3";
pub const DEFAULT_VST2_PATH: &str = "";

pub struct PluginConfig {
    vst2_path: PathBuf,
    vst3_path: PathBuf,
    platform: Platform,
}

impl Default for PluginConfig {
    fn default() -> Self {
        PluginConfig {
            vst3_path: PathBuf::from(DEFAULT_VST3_PATH),
            vst2_path: PathBuf::from(DEFAULT_VST2_PATH),
            platform: Platform::WINDOWS,
        }
    }
}

#[derive(Serialize, Debug, TS)]
#[ts(export)]
pub enum PluginType {
    Vst3,
    Vst2,
}

#[derive(Serialize, Debug, TS)]
#[ts(export)]
pub struct Plugin {
    pub plugin_type: PluginType,
    pub name: String,
    pub vendor: Option<String>,
    pub path_to_file: PathBuf,
}

#[derive(Serialize, Debug)]
pub enum Platform {
    WINDOWS,
    // MAC,
    // LINUX
}
