use petgraph::graph::{self, Node, NodeIndex, UnGraph};
use petgraph::visit::IntoEdges;
use petgraph::Direction;
use std::fmt;

#[derive(Debug)]
struct Fighter {
    name: String,
}

impl Fighter {
    fn new(name: String) -> Self {
        Self {
            name,
        }
    }    
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

fn main() {
    let mut graph = UnGraph::new_undirected();

    let fighters = [
        Fighter::new(String::from("Michael")),
        Fighter::new(String::from("Dungeon Master")),
        Fighter::new(String::from("Rey Misterio")),
        Fighter::new(String::from("Some kinda guy")),
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters
    .iter()
    .map(|fighter| graph.add_node(fighter))
    .collect();

    add_edge(&mut graph, &fighter_nodes, 0, 1);
    add_edge(&mut graph, &fighter_nodes, 1, 3);
    add_edge(&mut graph, &fighter_nodes, 3, 0);
    add_edge(&mut graph, &fighter_nodes, 2, 1);
    add_edge(&mut graph, &fighter_nodes, 2, 3);
    add_edge(&mut graph, &fighter_nodes, 0, 3);
    add_edge(&mut graph, &fighter_nodes, 1, 3);

    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        println!("The closeness centrality of {} is {:.2}", name, closeness);

        match name.as_str() {
            "Michael" => {
                println!(
                            "{} has centrality of {:.2}",
                            name, closeness
                )
            },
            "Dungeon Master" => {
                println!(
                    "{} has centrality of {:.2}",
                    name, closeness
                )
            },
            "Rey Misterio" => {
                println!(
                    "{} has centrality of {:.2}",
                    name, closeness
                )
            },
            "Some kinda guy" => {
                println!(
                    "{} has centrality of {:.2}",
                    name, closeness
                )
            },
            _ => {
                println!("Error")
            },
        }
    }
}
