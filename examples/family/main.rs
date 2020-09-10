use iso_10303::step::parser::exchange_file;

pub mod parser;
fn main() {
    let bytes = include_bytes!("family.stp");
    match exchange_file().parse(bytes) {
        Ok(file) => {
            println!("entities: {}", file.data.len());
            println!("{:?}", file);
        }
        Err(err) => println!("{:?}", err),
    }
}
