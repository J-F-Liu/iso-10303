use heck::*;
use iso_10303::express::*;

pub struct Generator {
    schema: Schema,
}

impl Generator {
    pub fn new(schema: Schema) -> Generator {
        Generator { schema }
    }

    pub fn gencode(&self) -> String {
        let mut code = String::new();
        code.push_str("digraph G {\n rankdir=LR;\n");
        for declaration in &self.schema.declarations {
            match declaration {
                Declaration::Entity(entity) => {
                    for supertype in &entity.supertypes {
                        code.push_str(&format!(
                            "  {} -> {};\n",
                            supertype.to_camel_case(),
                            entity.name.to_camel_case()
                        ));
                    }
                }
                _ => {}
            }
        }
        code.push_str("}\n");
        code
    }
}
