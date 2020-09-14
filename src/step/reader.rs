use super::parser::exchange_file;
use super::structure::*;
use std::path::Path;

pub trait StepReader {
    fn read_simple_entity(&mut self, id: i64, typed_parameter: TypedParameter);

    fn read<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        let bytes = std::fs::read(path)?;
        match exchange_file().parse(&bytes) {
            Ok(file) => {
                println!("entities: {}", file.data.len());
                for instance in file.data {
                    if instance.value.len() == 1 {
                        for typed_parameter in instance.value {
                            self.read_simple_entity(instance.id, typed_parameter);
                        }
                    }
                }
            }
            Err(err) => println!("{:?}", err),
        }

        Ok(())
    }
}
