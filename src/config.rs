pub struct SearchOptions {
    pub case_sensitive: bool,
    pub by_words: bool,
    pub replace_by: Option<String>,
}

impl SearchOptions {
    pub fn defaults() -> SearchOptions {
        SearchOptions { case_sensitive: true, by_words: false, replace_by: None }
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
        let mut options = SearchOptions::defaults();
        let args_count = args.len();
        for i in 3..args_count {
            match args[i].as_str() {
                "-i" => { options.case_sensitive = false; },
                "+w" => { options.by_words = true; },
                "+i" => { options.case_sensitive = true; },
                "-w" => { options.by_words = false; },
                "-r" => {
                    if i == args_count - 1 {
                        eprintln!("Invalid argument: Replace By option needs a second parameter: the replacement string!");
                    }
                    options.replace_by = Some(args[i+1].clone())
                }
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
