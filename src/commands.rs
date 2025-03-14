use std::process::Command;

pub fn execute_command(command: &str) {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    let output = Command::new(parts[0])
        .args(&parts[1..])
        .output();

    match output {
        Ok(output) => {
            if !output.stdout.is_empty() {
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
