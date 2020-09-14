use iso_10303::step::StepReader;

pub mod reader;
use reader::*;

fn main() {
    let bytes = include_bytes!("family.stp");
    // let bytes = include_bytes!("C:/Users/Liu/3D Objects/10mm光栅方头模型.stp");
    // let bytes = include_bytes!("C:/Users/Liu/3D Objects/HandySCAN 3D_Demo part_CAD.stp");

    match iso_10303::step::parser::exchange_file().parse(bytes) {
        Ok(file) => {
            println!("entities: {}", file.data.len());
            println!("{:?}", file.data);
        }
        Err(err) => println!("{:?}", err),
    }

    let mut reader = ExampleReader::new();
    if reader.read("examples/family/family.stp").is_ok() {
        for male in reader.get_entities::<Male>() {
            println!("{:?}", male);
        }
        for female in reader.get_entities::<Female>() {
            println!("{:?}", female);
        }
    }
}
