use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "Get" => Method::Get,
            "Post" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Patch(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Patch("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, version, resource) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
            } else {
                parsed_msg_body = line;
            }
        }
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            msg_body: parsed_msg_body.to_string(),
            headers: parsed_headers,
        }
    }
}

fn process_req_line(s: &str) -> (Method, Version, Resource) {
    let mut words = s.split_whitespace();
    let mut method = words.next().unwrap();
    let mut resource = words.next().unwrap();
    let mut version = words.next().unwrap();
    

    (
        method.into(),
        version.into(),
        Resource::Patch(resource.to_string()),
    )
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header_item = s.split(":");
    let mut key = String::from(" ");
    let mut value = String::from(" ");
    if let Some(k) = header_item.next() {
        key = k.to_string();
    }
    if let Some(v) = header_item.next() {
        value = v.to_string();
    }
    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "Get".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_method_into2() {
        let n: Version = "HTTP/1.1".into();
        assert_eq!(n, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s :String = String::from("Get /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n");
        let mut headers = HashMap::new();
        headers.insert("Host".into(), " localhost".into());
        headers.insert("User-Agent".into(), " curl/7.71.1".into());
        headers.insert("Accept".into(), " */*".into());
        let req :HttpRequest = s.into();
        
        assert_eq!(Method::Get,req.method);
        assert_eq!(Version::V1_1,req.version);
        assert_eq!(Resource::Patch("/greeting".to_string()),req.resource);
        assert_eq!(headers,req.headers);
    
    }
}
