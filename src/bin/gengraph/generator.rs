use heck::*;
use iso_10303::express::*;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet};

pub struct Generator {
    schema: Schema,
    nodes: HashMap<String, NodeIndex>,
    graph: DiGraph<String, ()>,
}

fn create_graph(schema: &Schema) -> (HashMap<String, NodeIndex>, DiGraph<String, ()>) {
    let mut graph = DiGraph::new();
    let mut nodes = HashMap::new();

    for declaration in &schema.declarations {
        match declaration {
            Declaration::Entity(entity) => {
                if entity.supertypes.len() > 0 {
                    let child_id = *nodes
                        .entry(entity.name.to_camel_case())
                        .or_insert(graph.add_node(entity.name.to_camel_case()));
                    for supertype in &entity.supertypes {
                        let parent_id = *nodes
                            .entry(supertype.to_camel_case())
                            .or_insert(graph.add_node(supertype.to_camel_case()));
                        graph.add_edge(parent_id, child_id, ());
                    }
                }
            }
            _ => {}
        }
    }
    (nodes, graph)
}

impl Generator {
    pub fn new(schema: Schema) -> Generator {
        let (nodes, graph) = create_graph(&schema);
        Generator { schema, nodes, graph }
    }

    fn write_edges(&self, parent_id: NodeIndex, visited: &mut HashSet<NodeIndex>, code: &mut String) {
        if visited.insert(parent_id) {
            let children = self
                .graph
                .neighbors(parent_id)
                .map(|child_id| self.graph.node_weight(child_id).unwrap())
                .fold(String::new(), |mut list, child| {
                    if list.len() > 0 {
                        list.push_str(", ");
                    }
                    list.push_str(child);
                    list
                });
            if children.len() > 0 {
                let parent = self.graph.node_weight(parent_id).unwrap();
                code.push_str(&format!("  {} -> {{{}}};\n", parent, children));
                for child_id in self.graph.neighbors(parent_id) {
                    self.write_edges(child_id, visited, code);
                }
            }
        }
    }

    pub fn gencode(&self, root: Option<String>) -> String {
        let mut code = String::new();
        let mut visited = HashSet::new();
        if let Some(root) = root {
            if let Some(parent_id) = self.nodes.get(&root) {
                code.push_str("digraph G {\n");
                self.write_edges(*parent_id, &mut visited, &mut code);
                code.push_str("}\n");
            } else {
                println!("{} not found", root);
            }
        } else {
            code.push_str("digraph G {\n rankdir=LR;\n");
            for node_id in self.nodes.values() {
                if self
                    .graph
                    .neighbors_directed(*node_id, petgraph::Direction::Incoming)
                    .count()
                    == 0
                {
                    self.write_edges(*node_id, &mut visited, &mut code);
                }
            }
            code.push_str("}\n");
        }
        code
    }
}
