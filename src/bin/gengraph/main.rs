use structopt::StructOpt;

mod generator;
use generator::Generator;
use iso_10303::express::parser;
use std::process::Command;

#[derive(StructOpt, Debug)]
struct Args {
    schema: std::path::PathBuf,
    dotfile: std::path::PathBuf,
    root: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();
    println!("work dir: {}", std::env::current_dir()?.display());
    println!("read file: {}", args.schema.display());
    let bytes = std::fs::read(args.schema)?;
    match parser::schema().parse(&bytes) {
        Ok(schema) => {
            // generate parser dot file
            println!("generate dot file");
            let generator = Generator::new(schema);
            let code = generator.gencode(args.root);

            // write code file
            println!("write file: {}", args.dotfile.display());
            std::fs::write(&args.dotfile, code.as_bytes())?;

            // generate svg image
            let svgfile = args.dotfile.with_extension("svg");
            Command::new("dot")
                .arg("-Tsvg")
                .arg(args.dotfile)
                .arg("-o")
                .arg(svgfile)
                .output()?;
        }
        Err(err) => println!("{:?}", err),
    }

    Ok(())
}
