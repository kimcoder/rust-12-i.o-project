use std::{fs, error::Error, io::Read};

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
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
  fn one_result() {
    let query = "Squat";
    let content = "\
Squat
TEST
rust";

    assert_eq!(vec!["Squat"], search(query, content));
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
