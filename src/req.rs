use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Method {
    Get,
    Post,
}

impl Method {
    pub fn from_string(value: &str) -> Method {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            method => panic!("Malformed method {:?}", method),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Req {
    pub path: String,
    pub method: Method,
    pub params: HashMap<String, String>
}

impl Req {
    pub fn new(path: String, method: Method, params: HashMap<String, String>) -> Req {
        Req {path, method, params}
    }

    pub fn from_string(req: String) -> Req {
        let mut params = HashMap::new();
        let index_of_newline = req.find('\n').unwrap_or(req.len());
        let header = req[0..index_of_newline].to_string();
        let req_params = req[(index_of_newline+2)..req.len()-1].to_string();

        let header: Vec<&str> = header.split(" ").collect();
        let method = Method::from_string(header[0]);
        let path = header[1].to_string();
        // let version = header[2].to_string();

        for line in req_params.split("\n") {
            if line == "\r" {
                // Final line break.
                break
            }
            let index_of_colon = line.find(':').unwrap_or(0);
            let head = line[0..index_of_colon].to_string();
            let tail = line[(index_of_colon+2)..line.len()-1].to_string();
            params.insert(head, tail);
        }
        Req::new(path, method, params)
    }
}
