pub mod graph {

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                node1: String,
                node2: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(node1: &str, node2: &str) -> Self {
                    Self {
                        node1: node1.to_string(),
                        node2: node2.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
                    Self {
                        node1: self.node1.to_owned(),
                        node2: self.node2.to_owned(),
                        attrs: attrs
                            .iter()
                            .map(|(attr, val)| (attr.to_string(), val.to_string()))
                            .collect(),
                    }
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|x| x.as_str())
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub label: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(label: &str) -> Self {
                    Self {
                        label: label.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
                    Self {
                        label: self.label.to_owned(),
                        attrs: attrs
                            .iter()
                            .map(|(attr, val)| (attr.to_string(), val.to_string()))
                            .collect(),
                    }
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|x| x.as_str())
                }
            }
        }
    }

    use std::collections::HashMap;

    use graph_items::edge::*;
    use graph_items::node::*;

    #[derive(Clone, PartialEq, Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(&self, nodes: &[Node]) -> Self {
            Self {
                nodes: nodes.to_owned(),
                edges: self.edges.to_owned(),
                attrs: self.attrs.to_owned(),
            }
        }

        pub fn with_edges(&self, edges: &[Edge]) -> Self {
            Self {
                nodes: self.nodes.to_owned(),
                edges: edges.to_owned(),
                attrs: self.attrs.to_owned(),
            }
        }

        pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
            Self {
                nodes: self.nodes.to_owned(),
                edges: self.edges.to_owned(),
                attrs: attrs
                    .iter()
                    .map(|(attr, val)| (attr.to_string(), val.to_string()))
                    .collect(),
            }
        }

        pub fn node(&self, label: &str) -> Option<Node> {
            self.nodes.iter().find(|node| node.label == label).cloned()
        }

        pub fn attr(&self, attr: &str) -> Option<&str> {
            self.attrs.get(attr).map(|x| x.as_str())
        }
    }
}
