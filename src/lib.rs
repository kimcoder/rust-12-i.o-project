use std::{fs, error::Error, io::Read, env};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        println!("CASE_SENSITIVE {}", case_sensitive);


        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = fs::File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
      println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "Squat";
    let content = "\
Squat
TEST
rust";

    assert_eq!(vec!["Squat"], search(query, content));
  }

  #[test]
  fn case_insensitive() {
    let query = "Squat";
    let content = "\
Squat
TEST
rust";

    assert_eq!(vec!["Squat"], search_case_case_insensitive(query, content));
  }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

pub fn search_case_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  results
}
