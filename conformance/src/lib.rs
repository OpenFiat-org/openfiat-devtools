//! Loads and structurally validates OpenFiat protocol conformance vectors.
//!
//! Vectors live in `vectors/<area>/*.json` (identity, settlement, trade,
//! reputation, governance, notifications) and are consumed by
//! implementations (starting with `openfiat-core`) to verify wire-format
//! and state-transition compatibility. This crate provides vector loading
//! only — it does not implement or judge conformance itself yet.

use std::fs;
use std::path::Path;

/// A single named conformance vector with its raw JSON payload.
#[derive(Debug, Clone)]
pub struct Vector {
    pub area: String,
    pub name: String,
    pub json: String,
}

/// Load every `*.json` vector under `vectors/<area>/`.
pub fn load_area(vectors_root: &Path, area: &str) -> std::io::Result<Vec<Vector>> {
    let dir = vectors_root.join(area);
    let mut out = Vec::new();
    if !dir.exists() {
        return Ok(out);
    }
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let json = fs::read_to_string(&path)?;
            let name = path.file_stem().unwrap().to_string_lossy().to_string();
            out.push(Vector {
                area: area.to_string(),
                name,
                json,
            });
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_trade_vectors() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("vectors");
        let vectors = load_area(&root, "trade").unwrap();
        assert!(!vectors.is_empty(), "expected at least one trade vector");
        for v in &vectors {
            let _: serde_json::Value = serde_json::from_str(&v.json)
                .unwrap_or_else(|e| panic!("{}/{} is not valid JSON: {e}", v.area, v.name));
        }
    }
}
