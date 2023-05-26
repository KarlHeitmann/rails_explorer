use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fmt::{Display, Formatter, Result as FmtResult};
use ratatui::text::{Span, Spans};

#[derive(Debug)]
pub struct RouteNode {
    domain: String,
    prefix: Option<String>,
    verb: String,
    uri_pattern: String,
    controller_action: String,
    original_line: String,
}

impl Display for RouteNode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f,
           "{}\n{}{}\n    PREFIX: {:?}\n    VERB: {}\n    CONTROLLER_ACTION: {}",
           self.original_line,
           self.domain,
           self.uri_pattern,
           self.prefix,
           self.verb,
           self.controller_action,
       )
    }
}

impl RouteNode {
    // pub fn new(domain: String, s: String) -> Result<Self, &'static str> {
    pub fn new(domain: String, s: String) -> Result<Self, String> {
        let s = s.trim();
        log::info!("{}", s);
        let original_line: String = s.to_string();
        /*
        let s.split_once(" ")
        // let ss, s = s.split(" ");
        */

        let ss = s.split(" ");
        let ss = ss.filter(|s| !s.is_empty() );
        match ss.clone().count() {
            5 => {
                let ss: Vec<&str> = ss.collect();
                Ok(Self {
                    domain,
                    prefix: Some(ss.get(0).unwrap().to_string()),
                    verb: ss.get(1).unwrap().to_string(),
                    uri_pattern: ss.get(2).unwrap().to_string(),
                    controller_action: ss.get(3).unwrap().to_string(),
                    original_line,
                })
            },
            4 => {
                let ss: Vec<&str> = ss.collect();
                Ok(Self {
                    domain,
                    prefix: Some(ss.get(0).unwrap().to_string()), verb: ss.get(1).unwrap().to_string(), uri_pattern: ss.get(2).unwrap().to_string(), controller_action: ss.get(3).unwrap().to_string(),
                    original_line,
                })
            },
            3 => {
                let ss: Vec<&str> = ss.collect();
                Ok(Self {
                    domain,
                    prefix: None, verb: ss.get(0).unwrap().to_string(), uri_pattern: ss.get(1).unwrap().to_string(), controller_action: ss.get(2).unwrap().to_string(),
                    original_line,
                })
            }
            0 => {
                Err(format!("Split has size 0"))
            }
            n => {
                // println!("ss: {}", s);
                Err(format!("Invalid number of strings in line {}", n))
            }
        }
    }
    pub fn original_line(&self) -> String {
        self.original_line.clone()
    }
    pub fn route(&self, target: &String) -> Result<String, &'static str> {
        if target.starts_with("app/views/") {
            let (_, file) = target.split_at(10);
            let (file_name, extension) = file.split_once(".").unwrap();
            let mut file_data: Vec<&str> = file_name.split("/").collect();
            let action = file_data.pop().unwrap();
            // println!("{:?}||||{}", file_data.join("::"), action);
            let target = format!("{}#{}", file_data.join("/"), action);

            match self.controller_action.contains(&target) {
                true => Ok(format!("{}{} | controller_action: {} | target: {}", self.domain, self.uri_pattern, self.controller_action, target)),
                false => Err("controller_action don't contain target"),
            }
        } else {
            Err("Target doesn't starts with `app/views`")
        }
    }
}

pub struct Routes {
    // path: String,
    domain: String,
    route_nodes: Vec<RouteNode>,
    pub length: usize,
}

impl Routes {
    fn parse_file(domain: &str, s: String) -> Vec<RouteNode> {
        let mut route_nodes: Vec<RouteNode> = vec![];
        let ss = s.split("\n");
        for s in ss {
            match RouteNode::new(domain.to_string(), s.to_string()) {
                Ok(route_node) => {
                    // println!("{}", route_node);
                    route_nodes.push(route_node)
                },
                // Err(_e) => {}
                Err(e) => {println!("ERR: {}", e)}
            }
        };
        route_nodes
    }

    pub fn new(domain: &str, routes_file_name: &str) -> Option<Self> {
        // Create a path to the desired file
        let path = Path::new(routes_file_name);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let routes = match File::open(&path) {
            Err(e) => 
            {
                // println!("Uh oh, something ||{}|| went wrong", e);
                None
            },
            Ok(mut file) => { 
                let mut s = String::new();
                match file.read_to_string(&mut s) {
                    Err(e) => { 
                        // println!("An error ||{}|| occurred", e);
                        None
                    },
                    Ok(_) => {
                        let route_nodes = Self::parse_file(domain, s);
                        let length = route_nodes.len();
                        Some(Self {
                            domain: domain.to_string(),
                            route_nodes,
                            length,
                        })
                    }
                }
            }
        };
        routes
    }

    pub fn get_original_lines_span(&self) -> Vec<Spans> {
        self.route_nodes.iter().map( |route_node| {
            Spans::from(route_node.original_line.clone())
        }).collect::<Vec<Spans>>()
    }

    pub fn get_node_route(&self, i: usize) -> String {
        // self.route_nodes.get(0).unwrap().into()
        // self.route_nodes.get(0).unwrap().into::<String>()
        format!("{}", self.route_nodes.get(i).unwrap())


    }

    pub fn find(&self, target: String) -> Option<String> {
        let mut result = None;
        for route_node in &self.route_nodes {
            match route_node.route(&target) {
                Ok(route) => {
                    result = Some(route);
                    break;
                },
                Err(_) => {},
            }
        }
        result
    }
}


