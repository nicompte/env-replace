extern crate colored;
extern crate dotenv;
extern crate regex;
extern crate structopt;

use colored::*;
use std::{
    env,
    fs::File,
    io::{self, prelude::*, BufReader},
    path::PathBuf,
    process,
};

use regex::Regex;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "env-replace")]
struct Opt {
    /// Regex e.g. "(%%([\w_]+)%%)"
    #[structopt(name = "REGEX")]
    regex: String,
    /// Input file
    #[structopt(name = "INPUT", parse(from_os_str))]
    input: PathBuf,
    /// Output file, defaults to the input file
    #[structopt(name = "OUTPUT", parse(from_os_str))]
    output: Option<PathBuf>,
    /// Verbose, will display found variables
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let opt = Opt::from_args();

    // open file
    let file = File::open(&opt.input)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    // compile provided regex
    let re = Regex::new(&opt.regex).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    let mut unfilled = vec![];
    let mut filled = vec![];

    // replace found occurences
    for cap in re.captures_iter(&contents.clone()) {
        match env::var(&cap[2]) {
            Ok(var) => {
                contents = contents.replace(&cap[1], &var);
                filled.push(cap[2].to_string());
            }
            Err(_) => {
                unfilled.push(cap[2].to_string());
            }
        }
    }

    // display replaced variables
    if opt.verbose {
        if !filled.is_empty() {
            if unfilled.is_empty() {
                println!(
                    "{} found and replaced environment variables:",
                    "✔".green()
                );
            } else {
                println!("{} found environment variables:", "✔".green());
            }
        }
        for f in &filled {
            println!(" - {}", f);
        }
    }

    // if not all environment variables are defined
    if !unfilled.is_empty() {
        println!("{} missing environment variables:", "✘".red());
        for u in &unfilled {
            println!(" - {}", u);
        }
        // exit with code 1, do not update the file
        println!(
            "\n{} not all expected variables are defined, the file will not be written",
            "✘".red()
        );
        process::exit(1);
    } else {
        // write the content in the file
        let output = opt.output.unwrap_or(opt.input);
        let mut f = File::create(&output)?;
        f.write_all(contents.as_bytes())?;
        println!(
            "{}{} content written in {}",
            if opt.verbose && !filled.is_empty() {
                "\n"
            } else {
                ""
            },
            "✔".green(),
            output.to_str().unwrap().cyan()
        );
    }
    Ok(())
}
