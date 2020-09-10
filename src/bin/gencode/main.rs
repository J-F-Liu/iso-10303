use structopt::StructOpt;

mod generator;
use generator::Generator;
use iso_10303::express::parser;
use std::process::Command;

#[derive(StructOpt, Debug)]
struct Args {
    schema: std::path::PathBuf,
    parser: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();
    println!("work dir: {}", std::env::current_dir()?.display());
    println!("read file: {}", args.schema.display());
    let bytes = std::fs::read(args.schema)?;
    match parser::schema().parse(&bytes) {
        Ok(schema) => {
            // generate parser code
            let generator = Generator::new(schema);
            let code = generator.gencode();

            // write code file
            println!("write file: {}", args.parser.display());
            std::fs::write(&args.parser, code.as_bytes())?;

            // format code file
            Command::new("rustfmt").arg(args.parser).output()?;
        }
        Err(err) => println!("{:?}", err),
    }

    Ok(())
}
