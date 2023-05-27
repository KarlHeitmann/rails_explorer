use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
// use std::fmt;

#[derive(Debug)]
pub enum RouteNodeNewErrorType {
    Header,
    BlackList,
    InvalidVectorSize,
    EmptyString,
}

pub struct RouteNodeNewError {
    pub code: RouteNodeNewErrorType,
    trimmed_line: String,
    original_line: String,
}

// Different error messages according to AppError.code
impl Display for RouteNodeNewError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let err_msg = match self.code {
            RouteNodeNewErrorType::Header => "The line to parse is the header.",
            RouteNodeNewErrorType::BlackList => "The line to parse is inside the blacklist.",
            RouteNodeNewErrorType::InvalidVectorSize => "The line to parse has an unrecognized number of words. Unable to parse.",
            RouteNodeNewErrorType::EmptyString => "The line to parse is empty."
        };
        write!(f, "RouteNodeNewErrorType: {}", err_msg)
    }
}

// A unique format for dubugging output
impl Debug for RouteNodeNewError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "RouteNodeNewError {{ code: {:?}, trimmed_line: {}, original_line {} }}",
            self.code, self.trimmed_line, self.original_line
        )
    }
}


#[derive(Debug)]
pub struct RouteNode {
    domain: String,
    prefix: Option<String>,
    verb: String,
    uri_pattern: String,
    controller_action: String,
    pub trimmed_line: String,
}

impl Display for RouteNode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f,
           "{}\n{}{}\n    PREFIX: {:?}\n    VERB: {}\n    CONTROLLER_ACTION: {}",
           self.trimmed_line,
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
    pub fn new(domain: String, s: String) -> Result<Self, RouteNodeNewError> {
        // let s = s.trim();
        let ss = s.trim().to_string();
        log::info!("{}", s);

        let ss = ss.split(" ");
        let ss = ss.filter(|s| !s.is_empty() );
        let trimmed_line: String = ss.clone().fold(String::new(), |sum, s| format!("{} {}", sum, s));
        if trimmed_line == String::from(" Prefix Verb URI Pattern Controller#Action") {
            // return Err(format!("Split has size 0"))
            return Err(RouteNodeNewError { code: RouteNodeNewErrorType::Header, original_line: s, trimmed_line })
        }
        // let trimmed_line: String = ss.to_string();
        match ss.clone().count() {
            5 => {
                let ss: Vec<&str> = ss.collect();
                let route_node = Self {
                    domain,
                    prefix: Some(ss.get(0).unwrap().to_string()),
                    verb: ss.get(1).unwrap().to_string(),
                    uri_pattern: ss.get(2).unwrap().to_string(),
                    controller_action: ss.get(3).unwrap().to_string(),
                    trimmed_line,
                };
                Ok(route_node)
            },
            4 => {
                let ss: Vec<&str> = ss.collect();
                let route_node = Self {
                    domain,
                    prefix: Some(ss.get(0).unwrap().to_string()),
                    verb: ss.get(1).unwrap().to_string(),
                    uri_pattern: ss.get(2).unwrap().to_string(),
                    controller_action: ss.get(3).unwrap().to_string(),
                    trimmed_line,
                };
                Ok(route_node)
            },
            3 => {
                let ss: Vec<&str> = ss.collect();
                let route_node = Self {
                    domain,
                    prefix: None,
                    verb: ss.get(0).unwrap().to_string(),
                    uri_pattern: ss.get(1).unwrap().to_string(),
                    controller_action: ss.get(2).unwrap().to_string(),
                    trimmed_line,
                };
                Ok(route_node)
            }
            0 => {
                Err(RouteNodeNewError { code: RouteNodeNewErrorType::EmptyString, original_line: s, trimmed_line})
            }
            n => {
                // println!("ss: {}", s);
                Err(RouteNodeNewError { code: RouteNodeNewErrorType::InvalidVectorSize, original_line: s, trimmed_line})
            }
        }
    }
    pub fn trimmed_line(&self) -> String {
        self.trimmed_line.clone()
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


