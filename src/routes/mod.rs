use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use ratatui::text::{Span, Spans};

use crate::routes::route_node::RouteNode;

pub struct Routes {
    // path: String,
    domain: String,
    route_nodes: Vec<RouteNode>,
    errors: Vec<String>,
    pub length: usize,
}

mod route_node;

impl Routes {
    fn parse_file(domain: &str, s: String) -> (Vec<RouteNode>, Vec<String>) {
        let mut route_nodes: Vec<RouteNode> = vec![];
        let mut errors: Vec<String> = vec![];
        let ss = s.split("\n");
        for s in ss {
            if s.is_empty() { continue; }
            match RouteNode::new(domain.to_string(), s.to_string()) {
                Ok(route_node) => {
                    // println!("{}", route_node);
                    route_nodes.push(route_node)
                },
                // Err(_e) => {}
                Err(e) => {
                    errors.push(s.to_string());
                    log::error!("Error while parsing line:\n{:?}\nDetails:\n{}", s, e);
                }
            }
        };
        (route_nodes, errors)
    }

    pub fn new(domain: &str, routes_file_name: &str) -> Option<Self> {
        // Create a path to the desired file
        let path = Path::new(routes_file_name);

        // Open the path in read-only mode, returns `io::Result<File>`
        let routes = match File::open(&path) {
            Err(e) => 
            {
                log::error!("Uh oh, something bad happened when trying to open the file \"{}\"\nDetail:\n{}", path.display(), e);
                None
            },
            Ok(mut file) => { 
                let mut s = String::new();
                match file.read_to_string(&mut s) {
                    Err(e) => { 
                        log::error!("An error occured while reading the file:\n{}", e); 
                        None
                    },
                    Ok(_) => {
                        let (route_nodes, errors) = Self::parse_file(domain, s);
                        let length = route_nodes.len();
                        Some(Self {
                            domain: domain.to_string(),
                            route_nodes,
                            errors,
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


