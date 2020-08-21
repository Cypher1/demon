#[derive(Debug, Clone)]
pub struct Req { pub path: String, pub method: String, pub params: std::collections::HashMap<String, String> }
impl Req {
    pub fn from_string(req: String) -> Req {
        let index_of_newline = req.find('\n').unwrap_or(req.len());
        let header = req[0..index_of_newline].to_string();
        let req_params = req[(index_of_newline+2)..req.len()-1].to_string();
        let header: Vec<&str> = header.split(" ").collect();
        let mut params = std::collections::HashMap::new();
        for line in req_params.split("\n") {
            if line == "\r" { break /* Final line break.*/  }
            let index_of_colon = line.find(':').unwrap_or(0);
            let head = line[0..index_of_colon].to_string();
            let tail = line[(index_of_colon+2)..line.len()-1].to_string();
            params.insert(head, tail);
        }
        Req{path: header[1].to_string(), method: header[0].to_string(), params}
    }
}
