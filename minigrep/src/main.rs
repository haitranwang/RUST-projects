use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = config.run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].to_string();
        let filename = args[2].to_string();

        Ok(Config { query, filename })
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents =
        fs::read_to_string(self.filename.to_string())?;
        println!("Content: {}", contents);

        Ok(())
    }
}
