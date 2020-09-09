use structopt::StructOpt;

mod generator;
use generator::Generator;
use iso_10303::express::parser;

#[derive(StructOpt, Debug)]
struct Args {
    schema: std::path::PathBuf,
    outdir: std::path::PathBuf,
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
            let outfile = args.outdir.join("parser.rs");
            println!("write file: {}", outfile.display());
            std::fs::write(&outfile, code.as_bytes())?;

            // format code file
            let config_toml = std::str::from_utf8(include_bytes!("../../../rustfmt.toml")).unwrap();
            let config = rustfmt::config::Config::from_toml(config_toml).unwrap();
            let summary = rustfmt::run(rustfmt::Input::File(outfile), &config);
            assert_eq!(false, summary.has_parsing_errors());
        }
        Err(err) => println!("{:?}", err),
    }

    Ok(())
}
