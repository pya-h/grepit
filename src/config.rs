pub struct SearchOptions {
    pub case_sensitive: bool,
    pub by_words: bool,
}

impl SearchOptions {
    pub fn defaults() -> SearchOptions {
        SearchOptions { case_sensitive: true, by_words: false }
    }
}
pub struct Config<'a> {
    query: &'a String,
    filename: &'a String,
    options: SearchOptions,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Config<'a>, String> {
        if args.len() < 3 {
            return Err(format!("[Usage] {} <query> <filename>", args[0]));
        }
        let mut options = SearchOptions {
            case_sensitive: true,
            by_words: false,
        };
        for i in 3..args.len() {
            match args[i].as_str() {
                "-i" => { options.case_sensitive = false; },
                "+w" => { options.by_words = true; },
                "+i" => { options.case_sensitive = true; },
                "-w" => { options.by_words = false; },
                _ => { 
                    eprintln!("Invalid argument: {}", args[i]);
                }
            }
        }
        Ok(Config {
            query: &args[1],
            filename: &args[2],
            options,
        })
    }

    pub fn to_string(&self) -> String {
        format!("{{query: {}, filename: {}}}", self.query, self.filename)
    }

    pub fn query(&self) -> String {
        self.query.clone()
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn options(&self) -> &SearchOptions {
        &self.options
    }
}
