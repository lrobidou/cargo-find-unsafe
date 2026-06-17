use std::io::Write;

/// Formats a piece of code
pub fn pretty_print(code: &str) -> String {
    // wraps the code in a function
    let wrapped = format!("fn dummy() {{\n{}\n}}", code);

    // format the code
    let mut child = std::process::Command::new("rustfmt")
        .arg("--emit")
        .arg("stdout")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn rustfmt");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(wrapped.as_bytes())
            .expect("Failed to write to rustfmt stdin");
    }

    let output = child
        .wait_with_output()
        .expect("Failed to read rustfmt output");
    let formatted = String::from_utf8_lossy(&output.stdout);

    // Strip the dummy function wrapper lines
    let lines: Vec<_> = formatted.lines().collect();
    if lines.len() >= 3 {
        lines[1..lines.len() - 1].join("\n")
    } else {
        // fallback: return entire formatted output if unexpected
        formatted.to_string()
    }
}
