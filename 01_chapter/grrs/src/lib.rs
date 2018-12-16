use std::env;

pub struct Query {
    pub pattern: String,
    pub path: String,
}

impl Query {
    //new will require env::Args and will produce a Result
    //When Ok it returns Query
    //When in Error it returns Err with error message
    pub fn new(mut args: env::Args) -> Result<Query, &'static str> {
        //first argument (0th) is the program name, should be ignored.
        args.next();

        //check if user provided pattern to search
        let pattern = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query stirng."),
        };
        //check if user provided path as the second argument
        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name."),
        };
        //if pattern and path were provided by the user, construct Query Struct
        Ok(Query { pattern, path })
    }
}

use std::error::Error;
use std::fs;
pub fn run(query: Query) -> Result<(), Box<dyn Error>> {
    //get the contents of the file
    let content = fs::read_to_string(query.path)?;
    //search for the result
    let results = search(&query.pattern, &content);
    //print result line by line
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
