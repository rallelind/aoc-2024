use std::process::Command;

fn main() {
    // List of days to execute
    let days = 1..=25;

    for day in days {
        let day_bin = format!("day{:02}", day);
        println!("\nRunning solution for {}", day_bin);

        // Execute the binary for the day
        let result = Command::new("cargo")
            .args(&["run", "-p", &day_bin])
            .output();

        match result {
            Ok(output) => {
                if !output.stdout.is_empty() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty() {
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(err) => {
                eprintln!("Failed to run {}: {}", day_bin, err);
            }
        }
    }
}
