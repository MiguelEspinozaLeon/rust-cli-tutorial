use anyhow::{Context, Result};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    //let pattern = std::env::args().nth(1).expect("no pattern given");
    //let path = std::env::args().nth(2).expect("no path given");


    /*let args = Cli {
        pattern,
        path: std::path::PathBuf::from(path),
    };*/

    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;

    /*let content = match result {
        Ok(content) => {content},
        Err(error) => {return Err(error.into());}
    };*/

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())

    /*for line in content.lines()  {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }*/

    //println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}

//todo: implement it using buffreader
