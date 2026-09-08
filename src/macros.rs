use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Macro {
    pub name: String,
    #[serde(default)]
    pub platform: String,
    #[serde(default)]
    pub steps: Vec<MacroStep>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MacroStep {
    pub module: String,
    #[serde(default)]
    pub options: std::collections::HashMap<String, String>,
}

pub fn load_dir(dir: &Path) -> Vec<Macro> {
    let mut out = Vec::new();
    let Ok(rd) = std::fs::read_dir(dir) else {
        return out;
    };
    for e in rd.flatten() {
        let p = e.path();
        if p.extension().and_then(|s| s.to_str()) != Some("toml") {
            continue;
        }
        if let Ok(s) = std::fs::read_to_string(&p) {
            if let Ok(m) = toml::from_str::<Macro>(&s) {
                out.push(m);
            }
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

pub fn save(dir: &Path, m: &Macro) -> anyhow::Result<()> {
    let path = dir.join(format!("{}.toml", m.name));
    std::fs::write(&path, toml::to_string_pretty(m)?)
        .with_context(|| format!("write {}", path.display()))
}

pub fn seed_defaults(dir: &Path) -> anyhow::Result<()> {
    if dir.join("default-windows.toml").exists() {
        return Ok(());
    }
    save(
        dir,
        &Macro {
            name: "default-windows".into(),
            platform: "windows".into(),
            steps: vec![
                MacroStep {
                    module: "post/windows/gather/enum_logged_on_users".into(),
                    options: Default::default(),
                },
                MacroStep {
                    module: "post/windows/gather/enum_computer".into(),
                    options: Default::default(),
                },
                MacroStep {
                    module: "post/multi/gather/env".into(),
                    options: Default::default(),
                },
            ],
        },
    )?;
    save(
        dir,
        &Macro {
            name: "default-linux".into(),
            platform: "linux".into(),
            steps: vec![
                MacroStep {
                    module: "post/linux/gather/enum_system".into(),
                    options: Default::default(),
                },
                MacroStep {
                    module: "post/linux/gather/enum_network".into(),
                    options: Default::default(),
                },
                MacroStep {
                    module: "post/multi/gather/env".into(),
                    options: Default::default(),
                },
            ],
        },
    )
}
