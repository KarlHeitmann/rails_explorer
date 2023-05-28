use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use ratatui::{
    text::{Span, Spans},
    widgets::{ListState, ListItem},
};


use crate::routes::route_node::{RouteNode, RouteNodeNewErrorType, RouteNodeNewError};

pub struct Routes {
    // path: String,
    domain: String,
    pub route_nodes: Vec<RouteNode>,
    errors: Vec<RouteNodeNewError>,
    pub length: usize,
}

pub mod route_node;

impl From<&RouteNode> for ListItem<'_> {
    fn from(route_node: &RouteNode) -> Self {
        ListItem::new(Spans::from(
            vec![
                Span::from(format!("{:100}{}", route_node.uri_pattern, route_node.controller_action)),
            ]
        ))
    }
}



impl Routes {
    fn parse_file(domain: &str, s: String) -> (Vec<RouteNode>, Vec<RouteNodeNewError>) {
        let mut route_nodes: Vec<RouteNode> = vec![];
        let mut errors: Vec<RouteNodeNewError> = vec![];
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
                    match e.code {
                        RouteNodeNewErrorType::InvalidVectorSize => {
                            log::error!("{}", e);
                            log::debug!("{:?}", e);
                            errors.push(e);
                        },
                        RouteNodeNewErrorType::EmptyString | RouteNodeNewErrorType::BlackList | RouteNodeNewErrorType::Header => {
                        },

                    }
                }
            }
        };
        (route_nodes, errors)
    }


    pub fn new(domain: &str, routes_file_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new(routes_file_name);

        let mut file = File::open(&path)?;
        let mut s = String::new();


        let _num_bytes_read = file.read_to_string(&mut s)?;
        let (route_nodes, errors) = Self::parse_file(domain, s);
        let length = route_nodes.len();
        Ok(Self {
            domain: domain.to_string(),
            route_nodes,
            errors,
            length,
        })
    }

    /*
    fn filter_route_nodes(&self, filter_string: String) -> &Vec<RouteNode> {
        self.route_nodes.into_iter()
            .filter(|route_node| route_node.uri_pattern.contains(&filter_string)).collect::<Vec<RouteNode>>()
    }
    */

    pub fn get_original_lines_span(&self, filter_string: &String) -> Vec<Spans> {
        self.route_nodes.iter()
            .filter_map(|route_node| match route_node.uri_pattern.contains(filter_string) {
                true => Some(Spans::from(format!("{:100}{}", route_node.uri_pattern, route_node.controller_action))),
                false => None
            }).collect::<Vec<Spans>>()
    }

    pub fn get_route_node(&self, i: usize, filter_string: &String) -> Option<&RouteNode> {
        self.route_nodes.iter()
            .filter(|route_node| route_node.uri_pattern.contains(filter_string))
            // .nth(i).unwrap("");
            .nth(i) /*{
                Some(route_node) => format!("{}", route_node),
                None => String::new()
            }*/
        
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


