use std::env;

pub struct Query {
    pub pattern: String,
    pub path: String,
}

impl Query {
    pub fn new(mut args: env::Args) -> Result<Query, &'static str> {
        args.next();

        let pattern = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query stirng."),
        };

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name."),
        };

        Ok(Query { pattern, path })
    }
}
