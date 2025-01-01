use rfd::FileDialog;
use std::{fs, process::Command};

fn main() {
    // Open dialog to select a folder
    let folder_path = FileDialog::new().set_directory(".").pick_folder();

    // Check if a folder was actually selected and closes the program if not.
    let folder_path = match folder_path {
        Some(path) => path,
        None => {
            eprintln!("No folder selected. Exiting.");
            std::thread::sleep(std::time::Duration::from_secs(2));
            return;
        }
    };

    println!("Selected folder: {}", folder_path.display());

    // Get all SVG files in the selected folder
    let svg_files: Vec<_> = match fs::read_dir(&folder_path) {
        Ok(entries) => entries
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let path = e.path();
                    if path.extension().and_then(|ext| ext.to_str()) == Some("svg") {
                        Some(path)
                    } else {
                        None
                    }
                })
            })
            .collect(),
        Err(err) => {
            eprintln!("Failed to read folder: {}", err);
            return;
        }
    };

    if svg_files.is_empty() {
        println!("No SVG files found in the selected folder.");
        return;
    }

    println!("Found {} SVG file(s).", svg_files.len());

    // Process each SVG file
    for svg_file in svg_files {
        let ico_file = svg_file.with_extension("ico");

        // Run the `magick` command
        let output = Command::new("magick")
            .arg("-background")
            .arg("none")
            .arg(&svg_file)
            .arg("-define")
            .arg("icon:auto-resize")
            .arg(&ico_file)
            .output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    eprintln!(
                        "Failed to process file {}: {}",
                        svg_file.display(),
                        String::from_utf8_lossy(&output.stderr)
                    );
                } else {
                    println!("Processed file: {}", svg_file.display());
                }

                // Validate that the .ico file was created
                if ico_file.exists() {
                    println!("File successfully created: {}", ico_file.display());
                } else {
                    eprintln!("File creation failed: {}", ico_file.display());
                }
            }
            Err(err) => {
                eprintln!(
                    "Error running command for file {}: {}",
                    svg_file.display(),
                    err
                );
            }
        }
    }
}
