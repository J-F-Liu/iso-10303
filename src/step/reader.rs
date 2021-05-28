use super::parser::exchange_file;
use super::structure::*;
use std::any::{Any, TypeId};
use std::path::Path;

pub trait StepReader {
    fn insert_entity(&mut self, id: i64, type_id: TypeId, type_name: &'static str, entity: Box<dyn Any>);
    fn create_simple_entity(
        &self,
        typed_parameter: TypedParameter,
        own_parameters_only: bool,
    ) -> Option<(TypeId, &'static str, Box<dyn Any>)>;
    fn create_complex_entity(
        &self,
        typed_parameters: Vec<TypedParameter>,
    ) -> Option<(TypeId, &'static str, Box<dyn Any>)> {
        let values = typed_parameters
            .into_iter()
            .filter_map(|typed_parameter| {
                if let Some((_, _, entity)) = self.create_simple_entity(typed_parameter, true) {
                    return Some(entity);
                }
                return None;
            })
            .collect::<Vec<Box<dyn Any>>>();
        let type_id = values.type_id();
        let type_name = std::any::type_name::<Vec<Box<dyn Any>>>();
        Some((type_id, type_name, Box::new(values)))
    }

    fn read<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        let bytes = std::fs::read(path)?;
        match exchange_file().parse(&bytes) {
            Ok(file) => {
                println!("entities: {}", file.data.len());
                for instance in file.data {
                    // println!("read #{}", instance.id);
                    if instance.value.len() == 1 {
                        for typed_parameter in instance.value {
                            if let Some((type_id, type_name, entity)) =
                                self.create_simple_entity(typed_parameter, false)
                            {
                                self.insert_entity(instance.id, type_id, type_name, entity);
                            }
                        }
                    } else {
                        if let Some((type_id, type_name, entity)) = self.create_complex_entity(instance.value) {
                            self.insert_entity(instance.id, type_id, type_name, entity);
                        }
                    }
                }
            }
            Err(err) => println!("{:?}", err),
        }

        Ok(())
    }
}
