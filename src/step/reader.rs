use super::parser::exchange_file;
use super::structure::*;
use std::any::{Any, TypeId};
use std::path::Path;

pub trait StepReader {
    fn insert_entity(&mut self, id: i64, type_id: TypeId, type_name: &'static str, entity: Box<dyn Any>);
    fn create_entity(&self, typed_parameter: TypedParameter) -> Option<(TypeId, &'static str, Box<dyn Any>)>;

    fn read<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        let bytes = std::fs::read(path)?;
        match exchange_file().parse(&bytes) {
            Ok(file) => {
                println!("entities: {}", file.data.len());
                for instance in file.data {
                    if instance.value.len() == 1 {
                        for typed_parameter in instance.value {
                            // println!("read #{}", instance.id);
                            if let Some((type_id, type_name, entity)) = self.create_entity(typed_parameter) {
                                self.insert_entity(instance.id, type_id, type_name, entity);
                            }
                        }
                    } else {
                        let values = instance
                            .value
                            .into_iter()
                            .filter_map(|typed_parameter| {
                                if let Some((_, _, entity)) = self.create_entity(typed_parameter) {
                                    Some(entity)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<Box<dyn Any>>>();
                        let type_id = values.type_id();
                        let type_name = std::any::type_name::<Vec<Box<dyn Any>>>();
                        self.insert_entity(instance.id, type_id, type_name, Box::new(values));
                    }
                }
            }
            Err(err) => println!("{:?}", err),
        }

        Ok(())
    }
}
