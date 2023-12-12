const WORKERD_PATH: &str = env!("WORKERD_PATH");

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
}
