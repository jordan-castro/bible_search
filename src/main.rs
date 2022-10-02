use std::fs;

mod pretty_print;

fn main() {
    let bible = Bible::load_bible();
    let error_messages = vec![
        "Book not found",
        "Chapter not found",
        "Verse not found",
        "Invalid lookup",
    ];

    loop {
        // Prompt user for input
        println!("Please enter a bible verse to search for: ");
        // Grab the input
        let mut query = String::new();
        std::io::stdin()
            .read_line(&mut query)
            .expect("Failed to read line");
        println!();
        // Trim the input
        let query = query.trim();
        let result = bible.search(query);
        // Get result
        println!("{}", &result);
        println!();

        // Only append if the result is a valid verse
        if !error_messages.contains(&result.as_str()) {
            // Write to file
            pretty_print::print("verses.txt".to_string(), result, 80);
        }

        // Ask user if they want to continue
        println!("Would you like to look up another verse? (y/n)");
        let mut continue_input = String::new();
        std::io::stdin()
            .read_line(&mut continue_input)
            .expect("Failed to read line");
        let continue_input = continue_input.trim().to_lowercase();
        if continue_input != "y" {
            break;
        }
        println!();
    }
}

struct Bible {
    books: Vec<Book>,
}

#[derive(Debug, PartialEq, Clone)]
struct Book {
    title: String,
    chapters: Vec<Chapter>,
    abreviations: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
struct Chapter {
    number: u32,
    verses: Vec<String>,
}

impl Chapter {
    fn empty_chapter() -> Chapter {
        Chapter {
            number: 0,
            verses: Vec::new(),
        }
    }

    /// Find a verse based on the verse number
    ///
    /// # Arguments
    /// - `verse_number` - The verse number to search for
    ///
    /// # Returns
    /// Option<String> - The verse if found, None otherwise
    fn verse_by_number(&self, verse_number: &i32) -> Option<String> {
        if verse_number < &1 {
            return None;
        }
        let verse_number = verse_number.clone() as usize;

        // Check the length of the verses vector
        if self.verses.len() < verse_number {
            return None;
        } else {
            // Subtract one because the verse numbers start at 1
            return Some(self.verses[verse_number - 1].clone());
        }
    }
}

impl Book {
    fn empty_book() -> Book {
        Book {
            title: String::new(),
            chapters: Vec::new(),
            abreviations: Vec::new(),
        }
    }

    fn load_abbreviations(&mut self) {
        // Read the abbrivations file
        let abreviations_file =
            fs::read_to_string("data/Bible_Abbreviations.csv").expect("Unable to read file");
        // Split the file into lines. Split by newline.
        let abreviations_file: Vec<&str> = abreviations_file.split("\n").collect();

        for line in abreviations_file {
            // Check if the line is empty.
            if line.is_empty() {
                // If empty skip to the next line.
                continue;
            }

            // Split the line by comma.
            let line: Vec<&str> = line.split(",").collect();
            // Check that the title is the same
            if &self.title.trim().to_lowercase() == &line[1].trim().to_lowercase() {
                // Add the line to the Vector.
                self.abreviations.push(line[0].trim().to_string());
            }
        }
    }

    /// Find a chapter by its number.
    fn chapter_by_number(&self, number: &i32) -> Option<Chapter> {
        if number < &1 {
            return None;
        }
        let number = number.clone();

        for chapter in &self.chapters {
            if chapter.number == number as u32 {
                return Some(chapter.clone());
            }
        }
        None
    }
}

impl Bible {
    fn load_books() -> Vec<Book> {
        // OPen he bible file
        let bible_file = fs::read_to_string("data/Bible.txt").expect("Unable to open bible file");
        // Split the file into lines by newline
        let bible_lines: Vec<&str> = bible_file.split("\n").collect();
        // Create a vector to hold the books
        let mut books: Vec<Book> = Vec::new();
        let mut current_book = Book::empty_book();
        let mut current_chapter = Chapter::empty_chapter();
        // Iterate over the lines
        for line in bible_lines {
            // Trim the line
            let line = line.trim();
            // Check if a start of a new book
            if line.starts_with("THE BOOK OF ") {
                // Check if we have a chapter to add to the book
                if current_chapter.number > 0 {
                    let clone = current_chapter.clone();
                    current_book.chapters.push(clone);
                }
                // Check if we have a book to add
                if current_book.chapters.len() > 0 {
                    let clone = current_book.clone();
                    // Add the book to the vector
                    books.push(clone);
                }
                let line = line.trim_start_matches("THE BOOK OF ");
                // Set the title and go to next iteration
                current_book.title = line.to_string();
                current_book.chapters.clear();
                current_book.abreviations.clear();
                current_book.load_abbreviations();
                continue;
            }
            // Check if a start of a new chapter
            if line.starts_with("CHAPTER ") || line.starts_with("PSALM ") {
                // Check if we have a chapter to add
                if current_chapter.number > 0 {
                    let clone = current_chapter.clone();
                    // Add it to the current book
                    current_book.chapters.push(clone);
                }
                // Split by space
                let line_number = line.split(" ").collect::<Vec<&str>>();
                let line_number = line_number[1].parse::<u32>().unwrap();
                // It is the start of a new chapter
                current_chapter.number = line_number;
                current_chapter.verses.clear();
            }

            if line.is_empty() {
                continue;
            }

            for number in "123456789".to_string().chars() {
                if line.starts_with(number) {
                    // It is a verse
                    let mut verse = line.clone();
                    // Remove each starting number in the verse
                    for vchar in verse.chars() {
                        // Only remove if number
                        if vchar.is_numeric() {
                            verse = verse.trim_start_matches(vchar);
                        } else {
                            // This means that there are no more STARTING numbers
                            break;
                        }
                    }

                    current_chapter.verses.push(verse.trim().to_string());
                }
            }
        }

        // Add the last book and chapter
        current_book.chapters.push(current_chapter);
        books.push(current_book);

        books
    }

    /// Load the Bible TXT file into a Vec<String> and return a Bible struct.
    fn load_bible() -> Self {
        let books = Self::load_books();

        Self { books }
    }

    fn book_by_title(&self, title: &str) -> Option<&Book> {
        for book in &self.books {
            // Check by title
            if book.title.to_lowercase() == title.to_lowercase() {
                return Some(book);
            }
            // Check by abbreviation
            for abbreviation in &book.abreviations {
                if abbreviation.to_lowercase() == title.to_lowercase() {
                    return Some(book);
                }
            }
        }
        None // Return None if no book is found
    }

    /// Search the bible for a verse.
    fn _search(&self, book_title: String, chapter_number: i32, verse_number: i32) -> String {
        // Check book exists
        let book = self.book_by_title(&book_title);
        if book.is_none() {
            return String::from("Book not found");
        }
        // Check chapter exists
        let chapter = book.unwrap().chapter_by_number(&chapter_number);
        if chapter.is_none() {
            return String::from("Chapter not found");
        }
        // Check verse exists
        let verse = chapter.clone().unwrap().verse_by_number(&verse_number);
        if verse.is_none() {
            return String::from("Verse not found");
        }
        
        // Format the book title, so that each word in the title has their first letter capitalized 
        let mut formatted_book_title = String::from("");
        let titles = book.unwrap().title.split(" ").collect::<Vec<&str>>();
        if titles.len() == 1 {
            formatted_book_title = capatilize_first_letter(titles[0].to_lowercase());
        } else {
            for x in 0..titles.len() {
                formatted_book_title.push_str(capatilize_first_letter(titles[x].to_string()).as_str());
                // Make sure to add a space after each word except the last one
                if x != titles.len() - 1 {
                    formatted_book_title.push_str(" ");
                }
            }
        }

        // Return the formatted verse
        format!(
            "{} {}:{} {}",
            formatted_book_title,
            chapter.unwrap().number,
            &verse_number,
            verse.unwrap()
        )
    }

    fn search(&self, query: &str) -> String {
        // Split the query by space
        let query: Vec<&str> = query.trim().split(" ").collect();
        if query.len() < 2 {
            return String::from("Invalid lookup");
        }
        // The chapter and verse number is the last element
        let chapter_verse = query.last().unwrap();
        // Split the chapter and verse by colon
        let chapter_verse: Vec<&str> = chapter_verse.split(":").collect();
        if chapter_verse.len() != 2 {
            return String::from("Invalid lookup");
        }
        // Get the chapter and verse number
        let chapter = chapter_verse[0].parse::<i32>().unwrap();
        let verse = chapter_verse[1].parse::<i32>().unwrap();
        // Get the book title
        let book_title = query[0..query.len() - 1].join(" ");

        self._search(book_title, chapter, verse)
    }
}

/// Captalize the first letter of a word.
///
/// # Params
/// - `word`: The word to capitalize.
///
/// # Returns
/// The capitalized word.
fn capatilize_first_letter(word: String) -> String {
    let mut capitalized_word = String::new();
    // Iterate over the word
    for (i, c) in word.to_lowercase().chars().enumerate() {
        // If the first letter
        if i == 0 {
            capitalized_word.push(c.to_ascii_uppercase());
        } else {
            capitalized_word.push(c);
        }
    }
    capitalized_word
}

#[cfg(test)]
mod tests {
    use crate::Bible;

    #[test]
    fn first_bible_verse() {
        let bible = Bible::load_bible();
        let result = bible.search("Genesis 1:1");
        let result = result.trim_start_matches("Genesis 1:1");
        assert_eq!(
            result,
            " In the beginning God created the heaven and the earth."
        );
    }

    #[test]
    fn book_not_found() {
        let bible = Bible::load_bible();
        let result = bible.search("JAB 42:17 ");
        let result = result.trim_start_matches("Jab 42:17");
        assert_eq!(result, "Book not found");
    }

    #[test]
    fn chapter_not_found() {
        let bible = Bible::load_bible();
        let result = bible.search("JOB 43:17 ");
        let result = result.trim_start_matches("Job 43:17");
        assert_eq!(result, "Chapter not found");
    }

    #[test]
    fn verse_not_found() {
        let bible = Bible::load_bible();
        let result = bible.search("JOB 42:18  ");
        let result = result.trim_start_matches("Job 42:18");
        assert_eq!(result, "Verse not found");
    }

    #[test]
    fn verse_in_psalms() {
        let bible = Bible::load_bible();
        let result = bible.search("PSALMS 3:5 ");
        let result = result.trim_start_matches("Psalms 3:5");
        assert_eq!(
            result,
            " I laid me down and slept; I awaked; for the LORD sustained me."
        );
    }

    #[test]
    fn multi_book_name() {
        let bible = Bible::load_bible();
        let result = bible.search("SONG OF SOLOMON 6:7 ");
        let result = result.trim_start_matches("Song Of Solomon 6:7");
        assert_eq!(
            result,
            " As a piece of a pomegranate [are] thy temples within thy locks."
        );
    }

    #[test]
    fn last_verse_in_chapter() {
        let bible = Bible::load_bible();
        let result = bible.search("MARK 16:20  ");
        let result = result.trim_start_matches("Mark 16:20");
        assert_eq!(result, " And they went forth, and preached every where, the Lord working with [them], and confirming the word with signs following. Amen.");
    }

    #[test]
    fn last_verse_in_bible() {
        let bilbe = Bible::load_bible();
        let result = bilbe.search("REVELATION 22:21 ");
        let result = result.trim_start_matches("Revelation 22:21");
        assert_eq!(
            result,
            " The grace of our Lord Jesus Christ [be] with you all. Amen."
        );
    }

    #[test]
    fn last_verse_in_genesis() {
        let bible = Bible::load_bible();
        let result = bible.search("GENESIS 50:26 ");
        let result = result.trim_start_matches("Genesis 50:26");
        assert_eq!(result, " So Joseph died, [being] an hundred and ten years old: and they embalmed him, and he was put in a coffin in Egypt.");
    }

    #[test]
    fn three_digit_chapter_and_verse() {
        let bible = Bible::load_bible();
        let result = bible.search("PSALMS 119:105  ");
        let result = result.trim_start_matches("Psalms 119:105");
        assert_eq!(
            result,
            " Thy word [is] a lamp unto my feet, and a light unto my path."
        );
    }

    #[test]
    fn non_existent_zero_verse() {
        let bilbe = Bible::load_bible();
        let result = bilbe.search("ISAIAH 42:0 ");
        let result = result.trim_start_matches("Isaiah 42:0");
        assert_eq!(result, "Verse not found");
    }

    #[test]
    fn verse_in_book_with_one_chapter() {
        let bible = Bible::load_bible();
        let result = bible.search("PHILEMON 1:1");
        let result = result.trim_start_matches("Philemon 1:1");
        assert_eq!(result, " Paul, a prisoner of Jesus Christ, and Timothy [our] brother, unto Philemon our dearly beloved, and fellowlabourer,");
    }

    #[test]
    fn test_two_word_book_name() {
        let bible = Bible::load_bible();
        let result = bible.search("FIRST PETER 3:5 ");
        let result = result.trim_start_matches("First Peter 3:5");
        assert_eq!(result, " For after this manner in the old time the holy women also, who trusted in God, adorned themselves, being in subjection unto their own husbands:");
    }

    #[test]
    fn test_non_existent_last_chapter_in_revelations() {
        let bible = Bible::load_bible();
        let result = bible.search("REVELATION 23:1  ");
        let result = result.trim_start_matches("Revelation 23:1");
        assert_eq!(result, "Chapter not found");
    }

    #[test]
    fn test_non_existent_last_verse_in_revelations() {
        let bible = Bible::load_bible();
        let result = bible.search("REVELATION 22:22 ");
        let result = result.trim_start_matches("Revelation 22:22");
        assert_eq!(result, "Verse not found");
    }
}
