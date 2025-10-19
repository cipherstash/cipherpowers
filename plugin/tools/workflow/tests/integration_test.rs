use std::fs;
use std::path::Path;

#[test]
fn test_no_arrow_syntax_in_examples() {
    let examples_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");

    for entry in fs::read_dir(examples_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let content = fs::read_to_string(&path).unwrap();

            // Old arrow syntax should not exist
            assert!(
                !content.contains("→ Exit 0:") && !content.contains("→ If output"),
                "File {:?} contains old arrow syntax (→)",
                path.file_name()
            );
        }
    }
}
