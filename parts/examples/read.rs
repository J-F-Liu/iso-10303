use iso_10303::step::StepReader;
use iso_10303_parts::ap214::*;

fn main() {
    let mut reader = AutomotiveDesignReader::new();
    match reader.read("examples/ap214_example.stp") {
        Ok(_) => {
            for organization in reader.get_entities::<Organization>() {
                println!("{:?}", organization);
            }
            for id in reader.entities.keys() {
                println!("{} - {}", id, reader.get_type_name(*id));
            }
        }
        Err(err) => println!("{:?}", err),
    }
}
