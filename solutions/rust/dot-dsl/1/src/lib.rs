pub mod graph {
    use graph_items::{edge::Edge, node::Node};
    use std::{
        collections::HashMap,
        fmt::{Display, Formatter, Result},
    };
    
    #[derive(Debug, PartialEq, Clone)]
    pub struct Attributes<'a>(HashMap<&'a str, &'a str>);
    impl<'a> Attributes<'a> {
        fn new() -> Self {
            Self(HashMap::new())
        }

        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }

    impl PartialEq<HashMap<String, String>> for Attributes<'_> {
        fn eq(&self, other: &HashMap<String, String>) -> bool {
            self.0
                .iter()
                .all(|(k, v)| other.get(&k.to_string()).map_or(false, |ov| v.eq(ov)))
        }
    }

    impl Display for Attributes<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let s = match self
                .0
                .iter()
                .map(|(k, v)| format!("{k}=\"{v}\""))
                .collect::<Vec<String>>()
            {
                e if e.is_empty() => String::new(),
                r => format!("[{}]", r.join(" ")),
            };
            f.write_str(&s)
        }
    }

    pub trait WithAttributes<'a> {
        fn get_attrs(&self) -> &Attributes<'a>;
        fn get_attrs_mut(&mut self) -> &mut Attributes<'a>;

        fn get_attr(&'a self, key: &str) -> Option<&str> {
            self.get_attrs().0.get(key).and_then(|&k| Some(k))
        }

        fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self
        where
            Self: Sized,
        {
            let self_attrs = self.get_attrs_mut();
            attrs.iter().for_each(|(ak, av)| {
                self_attrs.0.insert(ak, av);
            });
            self
        }
    }

    #[derive(Debug)]
    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: Attributes<'a>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: Attributes::new(),
            }
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name.eq(name))
        }

        pub fn with_nodes(mut self, nodes: &[Node<'a>]) -> Self {
            nodes.iter().cloned().for_each(|e| self.nodes.push(e));
            self
        }
        pub fn with_edges(mut self, edges: &[Edge<'a>]) -> Self {
            edges.iter().cloned().for_each(|e| self.edges.push(e));
            self
        }
        pub fn with_attrs(self, attrs: &[(&'a str, &'a str)]) -> Self {
            <Self as WithAttributes>::with_attrs(self, attrs)
        }
    }

    impl Display for Graph<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "graph {{\ngraph {}\n{}\n{}\n}}",
                self.attrs,
                self.nodes
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
                self.edges
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
    }

    impl<'a> WithAttributes<'a> for Graph<'a> {
        fn get_attrs(&self) -> &Attributes<'a> {
            &self.attrs
        }
        fn get_attrs_mut(&mut self) -> &mut Attributes<'a> {
            &mut self.attrs
        }
    }


    pub mod graph_items {
        pub mod edge {
            use std::fmt::Display;

            use crate::graph::{Attributes, WithAttributes};

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge<'a> {
                pub node_from: &'a str,
                pub node_to: &'a str,
                pub attrs: Attributes<'a>,
            }

            impl<'a> Edge<'a> {
                pub fn new(node_from: &'a str, node_to: &'a str) -> Self {
                    Self {
                        node_from: node_from,
                        node_to: node_to,
                        attrs: Attributes::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&'a str, &'a str)]) -> Self {
                    <Self as WithAttributes>::with_attrs(self, attrs)
                }
            }

            impl<'a> WithAttributes<'a> for Edge<'a> {
                fn get_attrs(&self) -> &Attributes<'a> {
                    &self.attrs
                }
                fn get_attrs_mut(&mut self) -> &mut Attributes<'a> {
                    &mut self.attrs
                }
            }

            impl Display for Edge<'_> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{} -- {} {}", self.node_from, self.node_to, self.attrs)
                }
            }
        }

        pub mod node {
            use std::fmt::Display;

            use crate::graph::{Attributes, WithAttributes};

            #[derive(Debug, PartialEq, Clone)]
            pub struct Node<'a> {
                pub(crate) name: &'a str,
                pub(crate) attrs: Attributes<'a>,
            }

            impl<'a> Node<'a> {
                pub fn new(name: &'a str) -> Self {
                    Self {
                        name,
                        attrs: Attributes::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&'a str, &'a str)]) -> Self {
                    <Self as WithAttributes>::with_attrs(self, attrs)
                }

                pub fn get_attr(&'a self, key: &str) -> Option<&str> {
                    <Self as WithAttributes<'a>>::get_attr(self, key)
                }
            }

            impl<'a> WithAttributes<'a> for Node<'a> {
                fn get_attrs(&self) -> &Attributes<'a> {
                    &self.attrs
                }
                fn get_attrs_mut(&mut self) -> &mut Attributes<'a> {
                    &mut self.attrs
                }
            }

            impl Display for Node<'_> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{} {}", self.name, self.attrs)
                }
            }
        }
    }
}
