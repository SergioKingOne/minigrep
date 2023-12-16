use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // Iterate over each line in the contents
    for line in contents.lines() {
        // Check if the line contains the query
        if line.contains(query) {
            // If it does, add it to the results
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Convert the query to lowercase
    let query = query.to_lowercase();

    let mut results = Vec::new();

    // Iterate over each line in the contents
    for line in contents.lines() {
        // Check if the line contains the query
        if line.to_lowercase().contains(&query) {
            // If it does, add it to the results
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn config_new_valid() {
        let args = vec![
            String::from("minigrep"),
            String::from("query"),
            String::from("file_path"),
        ];

        let config = Config::new(&args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file_path");
    }

    #[test]
    fn config_new_invalid() {
        let args = vec![String::from("minigrep"), String::from("query")];

        let config = Config::new(&args);

        assert!(config.is_err());
    }

    #[test]
    fn run_valid_file() {
        let query = "query";
        let file_path = "test.txt";
        let args = vec![
            String::from("minigrep"),
            query.to_string(),
            file_path.to_string(),
        ];

        let config = Config::new(&args).unwrap();

        // Create a file for the test
        let mut file = File::create(&config.file_path).unwrap();
        writeln!(file, "This is a test file.").unwrap();

        let result = run(config);

        assert!(result.is_ok());

        // Clean up
        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn run_invalid_file() {
        let query = "query";
        let file_path = "non_existent.txt";
        let args = vec![
            String::from("minigrep"),
            query.to_string(),
            file_path.to_string(),
        ];

        let config = Config::new(&args).unwrap();

        let result = run(config);

        assert!(result.is_err());
    }

    #[test]
    fn run_query_found() {
        let query = "query";
        let file_path = "test.txt";
        let args = vec![
            String::from("minigrep"),
            query.to_string(),
            file_path.to_string(),
        ];

        let config = Config::new(&args).unwrap();

        // Create a file for the test
        let mut file = File::create(&config.file_path).unwrap();
        writeln!(file, "This is a query test file.").unwrap();

        let result = run(config);

        assert!(result.is_ok());

        // Clean up
        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn run_query_not_found() {
        let query = "query";
        let file_path = "test.txt";
        let args = vec![
            String::from("minigrep"),
            query.to_string(),
            file_path.to_string(),
        ];

        let config = Config::new(&args).unwrap();

        // Create a file for the test
        let mut file = File::create(&config.file_path).unwrap();
        writeln!(file, "This is a test file.").unwrap();

        let result = run(config);

        assert!(result.is_ok());

        // Clean up
        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn search_query_in_contents() {
        let query = "safe";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_query_not_in_contents() {
        let query = "unsafe";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn case_insensitive_search() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive_search_no_match() {
        let query = "Swift";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(Vec::<&str>::new(), search_case_insensitive(query, contents));
    }
}
