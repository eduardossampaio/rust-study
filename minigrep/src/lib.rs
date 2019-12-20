
use std::error::Error;
use std::fs;
use std::env;

pub struct InputParams {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl InputParams {
   pub fn new(mut args: std::env::Args) -> Result<InputParams, &'static str> {
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }
        // // let query = args[1].clone();
        // // let filename = args[2].clone();
        // let args_it = args.iter().unwrap();
        // let query = args_it.next();
        // let filename = args_it.clone();

        args.next();//program name

        let query = match args.next() {
            Some(v) => v,
            None => return Err("Didn't get a query string")
        };

        let filename = match args.next(){
            Some(v) => v,
            None => return Err("Didn't get a file name")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(InputParams { query, filename,case_sensitive })
    }
}

pub fn run(input: InputParams) -> Result<(), Box<dyn Error>> {
    let contents =
    fs::read_to_string(input.filename).expect("Something went wrong reading the file");

    let result = if input.case_sensitive {
        search(&input.query, &contents)
    }else{
        search_case_insensitive(&input.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query){
    //         result.push(line);
    //     }
    // }
    // result
    contents.lines().filter(|x| x.contains(query) ).collect()
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();
    // let query = query.to_lowercase();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         result.push(line);
    //     }
    // }
    // result
    contents.lines().filter(|x| x.to_lowercase().contains(&query.to_lowercase())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
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
