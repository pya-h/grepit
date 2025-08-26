pub mod config;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

pub fn run(cfg: config::Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(cfg.filename())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File successfully read:\n\t{}\n", contents);
    let (l, v) = search(&cfg.query(), &contents, cfg.options());
    println!("{:?}, {:?}", l, v);
    Ok(())
}


fn extract_words(text: &str) -> Vec<&str> {
    if text.trim().is_empty() {
        return Vec::new();
    }
    match Regex::new(r"\w+") {
        Ok(re) => re.find_iter(text).map(|t| t.as_str()).collect(),
        _ => Vec::new(),
    }
}


pub fn search<'a>(
    query: &str,
    contents: &'a str,
    options: &config::SearchOptions,
) -> (Vec<usize>, Vec<&'a str>) {
    let mut results: Vec<&str> = Vec::new();
    let mut lines: Vec<usize> = Vec::new();
    if contents.trim().len() == 0 {
        panic!("No contents to search!");
    }
    if query.trim().len() == 0 {
        panic!("Empty query string provided!")
    }
    let query_lower = query.to_lowercase();
    let query = if options.case_sensitive {
        query
    } else {
        query_lower.as_str()
    }; // prevent query being lowered on each loop step
    let (query_in_words, query_words_count) = if options.by_words {
        let words = extract_words(query);
        (words.clone(), words.len())
    } else {
        (Vec::new(), 0)
    }; // In by_words case, This prevents the query from being re-generated everytime on each loop step
    for (line_number, line) in contents.lines().enumerate() {
        let matched: bool = match options.by_words {
            true => {
                let current_line = if options.case_sensitive { line.to_string() } else { line.to_lowercase() };
                let line_words = extract_words(current_line.as_str());
                let mut may_matched = false;
                let mut offset = 0;
                let last_word_index = line_words.len() - query_words_count + 1;
                while !may_matched && offset < last_word_index {
                    let mut ix = 0;
                    while ix < query_words_count && line_words[offset + ix] == query_in_words[ix] {
                        ix += 1;
                    }
                    may_matched = ix >= query_words_count;
                    offset += 1;
                }
                may_matched
            }
            false => match options.case_sensitive {
                true => line.contains(query),
                false => line.to_lowercase().contains(&query),
            },
        };
        if matched {
            results.push(line);
            lines.push(line_number);
        }
    }
    if lines.len() == 0 {
        panic!("No match found!");
    }
    (lines, results)
    // Notice: The reason that returning lines is not causing trouble (one might say lines is defined in the function,
    // and will be freed at the end), is that on the last line lines is moved, back to the calling function...
}

#[cfg(test)]
mod test {
    use crate::config::SearchOptions;

    use super::*;

    #[test]
    fn search_specific_line() {
        let query = "fast";
        let contents = "\
        Rust:
            safe, fast, productive.
            Pick three.";
        let (line_numbers, lines) = search(query, contents, &SearchOptions::defaults());
        // using different kinds of asserts
        assert_eq!(line_numbers.len(), 1);
        assert!(line_numbers == [1]);
        assert_ne!(lines.len(), 0);
        assert_eq!(lines[0].trim(), "safe, fast, productive.");
        assert!(lines.len() == 1)
    }

    #[test]
    #[should_panic(expected = "No match found!")]
    fn search_not_found() {
        search("X", "ABCDEFG", &SearchOptions::defaults());
    }

    #[test]
    #[should_panic(expected = "No contents to search!")]
    fn search_no_contents() {
        search("S", "", &SearchOptions::defaults());
    }

    #[test]
    #[should_panic(expected = "Empty query string")]
    fn search_empty_query() {
        search("", "Text", &SearchOptions::defaults());
    }
}
