use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use clap::Parser;
use glob::glob;
use std::io::{Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Query string to search for
    #[clap()]
    query: String,

    /// File Name to search
    #[clap()]
    file: String,

    /// ignore case when searching
    #[arg(short, long, default_value_t=false)]
    ignore_case: bool,

    /// Print filename
    #[arg(short, default_value_t=false)]
    filename:bool,
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub filename:bool,
}

impl Config {
    pub fn build(args: Args) -> Result<Config, &'static str> {

        let query = args.query.clone();
        let file_path = args.file.clone();
        let ignore_case = args.ignore_case.clone();
        let filename = args.filename.clone();

        Ok(Config { 
            query, 
            file_path, 
            ignore_case,
            filename, 
        })
    }
}

pub fn run(mut config: Config) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&config.file_path);

    if !path.exists(){
        let mut matching_files: Vec<PathBuf> = Vec::new();
        // let pattern = format!("{}*.txt", path.parent().unwrap().display());
        let pattern = format!("{}", &config.file_path);
        for entry in glob(&pattern)? {
            if let Ok(path) = entry {
                matching_files.push(path);
            }
        }
        if matching_files.is_empty() {
            return Err(format!("File '{}' does not exist.", path.display()).into());
        }
        config.filename = true;
        for file in matching_files {
            let contents = fs::read_to_string(&file)?;
            read_results(contents, &file.into_os_string().into_string().unwrap(), &config)?;
        }
    } else {
        let contents = fs::read_to_string(&config.file_path)?;
        read_results(contents, &config.file_path, &config)?;
    }
    Ok(())
}


pub fn read_results(contents: String, filename: &String, config: &Config) -> Result<(), Box<dyn Error>> {
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    // let stdout = io::stdout();
    let mut handle = StandardStream::stdout(ColorChoice::Always);

    for line in results {
        // let highlighted_line = String::new();
        let query_len = config.query.len();

        let mut start = 0;
        while let Some(pos) = line[start..].to_lowercase().find(&config.query) {
            if config.filename {
                write!(&mut handle, "{}: ", &filename)?;
            }
            write!(&mut handle, "{}", &line[start..start + pos])?;
            handle.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            write!(&mut handle, "{}", &line[start + pos..start + pos + query_len])?;
            handle.reset()?;

            start += pos + query_len;
        }
        writeln!(&mut handle, "{}", &line[start..])?;
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            //do something
            // let l = format!("{}: {}", (i + 1).to_string(), line);
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}