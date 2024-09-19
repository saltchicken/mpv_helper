use std::env;
use std::process::Command;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a video file is provided
    if args.len() < 2 {
        eprintln!(
            "Usage: {} <video_file> [geometry_x] [geometry_y] [crop_x] [crop_y]",
            args[0]
        );
        std::process::exit(1);
    }

    let video_file = &args[1];

    // Set default values
    let geometry_x = args
        .get(2)
        .map_or_else(|| "960".to_string(), |s| s.to_owned());
    let geometry_y = args
        .get(3)
        .map_or_else(|| "540".to_string(), |s| s.to_owned());
    let crop_x = args
        .get(4)
        .map_or_else(|| "0".to_string(), |s| s.to_owned());
    let crop_y = args
        .get(5)
        .map_or_else(|| "0".to_string(), |s| s.to_owned());

    // Command to run
    let output = Command::new("mpv")
        .arg("--loop")
        .arg(video_file)
        .arg("--no-border")
        .arg(format!("--geometry={}x{}+0+0", geometry_x, geometry_y))
        .arg(format!(
            "--vf=crop={}:{}:{}:{}",
            geometry_x, geometry_y, crop_x, crop_y
        ))
        .output() // Execute the command
        .expect("Failed to execute command");

    // Convert the output to a string
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Print the output
    println!("Output:\n{}", stdout);
    if !stderr.is_empty() {
        println!("Error:\n{}", stderr);
    }
}
