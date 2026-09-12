use regex::Regex;

use crate::config;

pub struct SingleMatch {
    pub line: String,
    pub line_number: usize,
    pub position: usize,
}

impl SingleMatch {
    pub fn new(line: String, line_number: usize, position: usize) -> Self {
        SingleMatch {
            line: line,
            line_number,
            position,
        }
    }

    pub fn to_string(&self) -> String {
        format!("[{}, {}] {}", self.line_number, self.position, self.line)
    }
}

pub struct GiMatches {
    pub items: Vec<SingleMatch>,
    pub replacements: Option<Vec<String>>,
}

impl GiMatches {
    pub fn new() -> Self {
        GiMatches {
            items: Vec::new(),
            replacements: None,
        }
    }

    pub fn next(&mut self, line: &str, line_number: usize, position: usize) -> &SingleMatch {
        self.items
            .push(SingleMatch::new(line.to_owned(), line_number, position));
        &self.items[self.items.len() - 1]
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
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

    pub fn findall<'a>(
        query: &str,
        contents: &'a str,
        options: &config::SearchOptions,
    ) -> GiMatches {
        let mut results = GiMatches::new();
        if contents.trim().len() == 0 {
            panic!("No contents to search!");
        }
        if query.trim().len() == 0 {
            panic!("Empty query string provided!")
        }
        let query = &options.get_phrase(query); // prevent query being lowered on each loop step
        let query_words_extracted = if options.by_words {
            let words = GiMatches::extract_words(query);
            Some((words.len(), words))
        } else {
            None
        }; // In by_words case, This prevents the query from being re-generated everytime on each loop step
        for (line_number, line) in contents.lines().enumerate() {
            let matched: Option<usize> = match query_words_extracted {
                Some((q_count, ref q_words)) => {
                    let current_line = &options.get_phrase(line);
                    let line_words = GiMatches::extract_words(current_line.as_str());
                    let mut offset = 0;
                    let last_word_index = line_words.len() - q_count;
                    while offset <= last_word_index {
                        let mut ix = 0;
                        while ix < q_count && line_words[offset + ix] == q_words[ix] {
                            ix += 1;
                        }
                        if ix >= q_count {
                            break;
                        }
                        offset += 1;
                    }
                    if offset <= last_word_index {
                        Some(offset)
                    } else {
                        None
                    }
                }
                _ => match options.case_sensitive {
                    true => line.find(query),
                    false => line.to_lowercase().find(query),
                },
            };
            if let Some(position) = matched {
                results.next(&line, line_number, position);
            }
        }
        if results.is_empty() {
            panic!("No match found!");
        }
        results
    }

    pub fn report(&self) -> Vec<String> {
        self.items.iter().map(|it| it.to_string()).collect()
    }

    pub fn line_numbers(&self) -> Vec<usize> {
        self.items.iter().map(|it| it.line_number).collect()
    }

    pub fn lines(&self) -> Vec<&String> {
        self.items.iter().map(|it| &it.line).collect()
    }
    
    pub fn zip(&self) -> Vec<(usize, &String)> {
        self.items.iter().map(|it| it.line_number).zip(self.items.iter().map(|it| &it.line)).collect()
    }
}


#[cfg(test)]
mod test {
    use crate::config::SearchOptions;
    use super::GiMatches;

    #[test]
    fn search_specific_line() {
        let query = "fast";
        let contents = "\
        Rust:
            safe, fast, productive.
            Pick three.";
        let matches = GiMatches::findall(query, contents, &SearchOptions::defaults());
        // using different kinds of asserts
        assert_eq!(matches.count(), 1);
        assert!(line_numbers == [1]);
        assert_ne!(lines.len(), 0);
        assert_eq!(lines[0].trim(), "safe, fast, productive.");
        assert!(lines.len() == 1)
    }

    #[test]
    #[should_panic(expected = "No match found!")]
    fn search_not_found() {
        GiMatches::findall("X", "ABCDEFG", &SearchOptions::defaults());
    }

    #[test]
    #[should_panic(expected = "No contents to search!")]
    fn search_no_contents() {
        GiMatches::findall("S", "", &SearchOptions::defaults());
    }

    #[test]
    #[should_panic(expected = "Empty query string")]
    fn search_empty_query() {
        GiMatches::findall("", "Text", &SearchOptions::defaults());
    }
}
