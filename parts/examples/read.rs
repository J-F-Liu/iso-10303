use iso_10303::step::StepReader;
use iso_10303_parts::ap214::*;

fn main() {
    // let mut reader = Ap203Reader::new();
    let mut reader = Ap214Reader::new();
    match reader.read("examples/ap214_example.stp") {
        Ok(_) => {
            for context in reader.get_entities::<ApplicationContext>() {
                println!("{:?}", context);
            }
            let mut total = 0;
            for (type_id, entity_ids) in reader.type_ids {
                println!("{:?} - {} ({})", type_id, reader.type_names[&type_id], entity_ids.len());
                total += entity_ids.len();
            }
            println!("Total: {}", total);
        }
        Err(err) => println!("{:?}", err),
    }
}
