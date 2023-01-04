use clap::{Parser};
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
        //.map_err(|err| CustomError(format!("Error reading `{}`: {}", args.path.display(), err)))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    // if let Ok(file) = std::fs::File::open(&args.path) {
    //     let mut reader = std::io::BufReader::new(file);

    //     let mut buf: String = String::new();
    //     match reader.read_to_string(&mut buf) {
    //         Ok(_) => {
    //             for line in buf.lines() {
    //                 if line.contains(&args.pattern) {
    //                     println!("{}", line);
    //                 }
    //             }
    //         },
    //         Err(error) => {
    //             println!("Could not read file");
    //             return Err(error.into());
    //         },
    //     }
    // }

    println!("{}", args.pattern);
    println!("{}", args.path.display());

    Ok(())

}
