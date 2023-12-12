use std::process::Command;

use anyhow::{anyhow, Result};

const WORKERD_PATH: &str = env!("WORKERD_PATH");

pub struct Workerd {}

impl Workerd {
    pub fn version() -> Result<String> {
        let output = Command::new(WORKERD_PATH).arg("--version").output()?;
        let output_str = String::from_utf8_lossy(&output.stdout);
        let raw_version = output_str
            .split_whitespace()
            .last()
            .ok_or(anyhow!("Cannot parse workerd version output"))?;
        Ok(format!("1.{}.0", raw_version.replace("-", "")))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_workerd_path_windows() {
        let mut path = PathBuf::from(env!("OUT_DIR"));
        path.push("package");
        path.push("bin");
        path.push("workerd.exe");
        assert_eq!(WORKERD_PATH, path.to_str().unwrap());
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_workerd_path_others() {
        let mut path = PathBuf::from(env!("OUT_DIR"));
        path.push("package");
        path.push("bin");
        path.push("workerd");
        assert_eq!(WORKERD_PATH, path.to_str().unwrap());
    }

    #[test]
    fn test_workerd_version() {
        let version = Workerd::version().unwrap();
        assert_eq!(version, env!("CARGO_PKG_VERSION"));
    }
}
