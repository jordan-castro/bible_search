use std::io::Write;

// The pretty print module.

/// Pretty Print. Writes to a file the line passed and breaks the line by lenght.
/// The line will only ever be shorter than lenght, never longer.
///
/// # Params
/// - `file_path`: The file path to write to.
/// - `line`: The line to write.
/// - `lenght`: The lenght of the line.
pub fn print(file_path: String, line: String, length: u32) {
    // Check if the file exists.

    // Open the file.
    let mut file = create_file_if_not_exists(file_path.clone());
    // The lines that will be written to the file.
    let mut lines = Vec::new();
    // Get the length of the line
    let line_length = line.len();
    // If the line is longer than the length, break it up.
    if line_length > length as usize {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let mut line = String::new();
        let mut current_line_length = 0;
        // Loop through the words by index.
        for x in 0..words.len() {
            // Get word length add one for the space.
            let word_length = words[x].len() + 1;
            current_line_length += word_length;
            // Check if the word would go over the line length. or if the word is the last word.
            if current_line_length > length as usize || x == words.len() - 1 {
                if x == words.len() - 1 {
                    // If the word is the last word, add it to the line.
                    line.push_str(words[x]);
                }
                // If it would, add the line to the lines vector.
                lines.push(line);
                // Reset the line.
                line = String::new();
                // Reset the line length.
                current_line_length = 0;
            }
            line.push_str(format!("{} ", words[x]).as_str());
        }
    } else {
        // Add the line to the lines vector.
        lines.push(line);
    }

    // Write the lines to the file.
    for line in lines {
        match file.write(&line.as_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", &file_path, why.to_string()),
            Ok(_) => (),
        }
        // Add a new line.
        match file.write("\n".as_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", &file_path, why.to_string()),
            Ok(_) => (),
        }
    }

    // Add a new line.
    match file.write("\n".as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", &file_path, why.to_string()),
        Ok(_) => (),
    }
}

/// Create a file.
/// 
/// # Params
/// - `file_path`: The file path to create.
/// 
/// # Returns
/// - `file`: The file that was created.
fn create_file_if_not_exists(file_path: String) -> std::fs::File {
    // Check if the file exists.
    if !std::path::Path::new(&file_path).exists() {
        // If it doesn't, create it.
        std::fs::File::create(&file_path).expect("Unable to create file");
    }
    // Open the file.
    std::fs::OpenOptions::new()
        .append(true)
        .open(&file_path)
        .expect("Unable to open file")
}