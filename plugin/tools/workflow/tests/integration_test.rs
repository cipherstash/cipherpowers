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

#[test]
fn test_enforcement_example_executable() {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let example_path = Path::new(manifest_dir).join("examples/enforcement.md");

    // Build the workflow binary first
    let build_status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(manifest_dir)
        .status()
        .expect("Failed to build workflow");

    assert!(build_status.success(), "Build failed");

    // Run the enforcement example with 'y' input for prompts
    let binary_path = Path::new(manifest_dir).join("target/release/workflow");

    let mut child = Command::new(binary_path)
        .arg(example_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn enforcement example");

    // Provide 'y' answer to the prompt
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"y\n").expect("Failed to write to stdin");
    }

    let output = child
        .wait_with_output()
        .expect("Failed to wait for enforcement example");

    // Should complete successfully (all commands succeed)
    assert!(
        output.status.success(),
        "Enforcement example failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_guided_example_syntax_valid() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let example_path = Path::new(manifest_dir).join("examples/guided.md");

    let content = fs::read_to_string(&example_path).unwrap();

    // Verify Pass/Fail syntax present (not arrow syntax)
    assert!(
        content.contains("Pass:") || content.contains("Fail:"),
        "Guided example should use Pass/Fail syntax"
    );
    assert!(
        !content.contains("→ Exit 0:"),
        "Guided example should not use arrow syntax"
    );

    // Verify it has GoTo conditional (not just text mention)
    assert!(
        content.contains("Pass: Go to Step") || content.contains("Fail: Go to Step"),
        "Guided example should have GoTo conditional"
    );
}

#[test]
fn test_simple_example_executable() {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let example_path = Path::new(manifest_dir).join("examples/simple.md");

    // Build the workflow binary first
    let build_status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(manifest_dir)
        .status()
        .expect("Failed to build workflow");

    assert!(build_status.success(), "Build failed");

    // Run the simple example
    let binary_path = Path::new(manifest_dir).join("target/release/workflow");

    let mut child = Command::new(binary_path)
        .arg(example_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn simple example");

    // Provide 'y' answer to the prompt
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"y\n").expect("Failed to write to stdin");
    }

    let output = child
        .wait_with_output()
        .expect("Failed to wait for simple example");

    // Should complete successfully (all echo commands succeed)
    assert!(
        output.status.success(),
        "Simple example failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
}
