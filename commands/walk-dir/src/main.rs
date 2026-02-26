use std::fs;

fn main() {
    // Read current directory entries
    let entries = match fs::read_dir(".") {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Failed to read directory: {}", e);
            return;
        }
    };

    // Iterate and print each file name
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Failed to read entry: {}", e);
                continue;
            }
        };

        let path = entry.path();
        // Print the file name
        println!("{}", path.display());
    }
}
