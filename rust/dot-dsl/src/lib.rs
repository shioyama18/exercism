#[macro_use]
extern crate maplit;

pub mod graph {
    use graph::graph_items::edge::Edge;
    use graph::graph_items::node::Node;
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: hashmap![],
            }
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            Graph {
                nodes: Vec::from(nodes),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            Graph {
                edges: Vec::from(edges),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            Graph {
                attrs: attrs.iter()
                    .map(|(k, v)| (String::from(*k), String::from(*v)))
                    .collect(),
                ..self
            }
        }

        pub fn get_node(&self, name: &str) -> Option<Node> {
            self.nodes.iter().find(|n| n.get_name() == name).cloned()
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(PartialEq, Eq, Clone, Debug)]
            pub struct Edge {
                start: String,
                end: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(start: &str, end: &str) -> Edge {
                    Edge {
                        start: String::from(start),
                        end: String::from(end),
                        attrs: hashmap![],
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Edge {
                        attrs: attrs.iter()
                            .map(|(k, v)| (String::from(*k), String::from(*v)))
                            .collect(),
                        ..self
                    }
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(PartialEq, Eq, Clone, Debug)]
            pub struct Node {
                name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Node {
                    Node {
                        name: String::from(name),
                        attrs: hashmap![],
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Node {
                        attrs: attrs.iter()
                            .map(|(k, v)| (String::from(*k), String::from(*v)))
                            .collect(),
                        ..self
                    }
                }

                pub fn get_name(&self) -> &str {
                    &self.name
                }

                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|s| s.as_str())
                }
            }
        }
    }
}
