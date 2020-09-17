use iso_10303::step::StepReader;
use iso_10303_parts::ap214::*;

fn main() {
    let mut reader = AutomotiveDesignReader::new();
    match reader.read("examples/ap214_example.stp") {
        Ok(_) => {
            for organization in reader.get_entities::<Organization>() {
                println!("{:?}", organization);
            }
            for product in reader.get_entities::<Product>() {
                println!("{:?}", product);
            }
        }
        Err(err) => println!("{:?}", err),
    }
}
