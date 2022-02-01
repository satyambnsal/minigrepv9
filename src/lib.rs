/// # Minigrep
///
/// `minigrep` is a collection of utilities to imitate `grep` command in unix
///
/// // --snip=
use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
  query: String,
  filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new<T>(mut args: T) -> Result<Config, &'static str>
  where
    T: Iterator<Item = String>,
  {
    args.next();
    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };
    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file name"),
    };
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };
  for line in results {
    println!("{}", line);
  }
  // println!("With text:\n{}", contents);
  Ok(())
}
/// search matching lines with specified word in string content
/// # Example
/// ```
/// let query = "you";
/// let contents = "test you\nhello";
/// let result = minigrepv9::search(query, contents);
/// assert_eq!(vec!["test you"], result);
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  // let mut result = Vec::new();

  // for line in contents.lines() {
  //   if line.contains(query) {
  //     result.push(line);
  //   }
  // }
  // result

  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  // let mut results = Vec::new();

  // for line in contents.lines() {
  //   if line.to_lowercase().contains(&query) {
  //     results.push(line);
  //   }

  contents
    .lines()
    .filter(|line| {
      let line = line.to_lowercase();
      line.contains(&query)
    })
    .collect()

  // }

  // results
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_new_config() {
    let original_args = vec!["sample", "test", "poem.txt"];
    let itr = original_args.iter().map(|s| s.to_string());
    let config = Config::new(itr).unwrap();
    println!("{:?}", config);
    assert_eq!("test", config.query);
  }

  #[test]
  fn one_result() {
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
