use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn source_files_stay_under_200_lines() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut failures = Vec::new();
    collect_source_files(&root, &mut failures);

    let too_long: Vec<String> = failures
        .into_iter()
        .filter_map(|path| {
            let text = fs::read_to_string(&path).ok()?;
            let lines = text.lines().count();
            (lines > 200).then(|| format!("{} has {lines} lines", path.display()))
        })
        .collect();

    assert!(too_long.is_empty(), "{}", too_long.join("\n"));
}

fn collect_source_files(path: &Path, files: &mut Vec<PathBuf>) {
    if !path.exists() {
        return;
    }
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().and_then(|name| name.to_str()) != Some("target") {
                collect_source_files(&path, files);
            }
        } else if is_source_file(&path) {
            files.push(path);
        }
    }
}

fn is_source_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("rs" | "py" | "js" | "ts" | "go")
    )
}
