use iso_10303::step::StepReader;
use iso_10303_parts::ap203;
use iso_10303_parts::ap214;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let input_file = &args[1];
    let instant = std::time::Instant::now();
    let mut parsing_time = 0.0;
    let mut reader = ap214::Ap214Reader::new();
    match reader.read(input_file) {
        Ok(_) => {
            parsing_time = instant.elapsed().as_secs_f64();
            for context in reader.get_entities::<ap214::ApplicationContext>() {
                println!("{:?}", context);
            }
            let mut total = 0;
            for (type_id, entity_ids) in reader.type_ids {
                println!("{:?} - {} ({})", type_id, reader.type_names[&type_id], entity_ids.len());
                total += entity_ids.len();
            }
            println!("simple entities: {}", total);
        }
        Err(err) => println!("{:?}", err),
    }
    println!("elapsed time: {} seconds", parsing_time);

    // let mut reader = ap203::Ap203Reader::new();
    // match reader.read("examples/ap203_example.stp") {
    //     Ok(_) => {
    //         for context in reader.get_entities::<ap203::ApplicationContext>() {
    //             println!("{:?}", context);
    //         }
    //         let mut total = 0;
    //         for (type_id, entity_ids) in reader.type_ids {
    //             println!("{:?} - {} ({})", type_id, reader.type_names[&type_id], entity_ids.len());
    //             total += entity_ids.len();
    //         }
    //         println!("simple entities: {}", total);
    //     }
    //     Err(err) => println!("{:?}", err),
    // }
}
