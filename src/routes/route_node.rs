use std::fmt::{Display, Formatter, Result as FmtResult};
#[derive(Debug)]
pub struct RouteNode {
    domain: String,
    prefix: Option<String>,
    verb: String,
    uri_pattern: String,
    controller_action: String,
    pub original_line: String,
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


