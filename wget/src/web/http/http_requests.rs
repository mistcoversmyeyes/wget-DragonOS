use std::{error::Error, fmt::Display};


pub enum HttpProtocolVersion {
    Http09,
    Http10,
    Http11,
    Http20,
    Http30, 
}

impl Display for HttpProtocolVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HttpProtocolVersion::Http09 => "HTTP/0.9",
            HttpProtocolVersion::Http10 => "HTTP/1.0",
            HttpProtocolVersion::Http11 => "HTTP/1.1",
            HttpProtocolVersion::Http20 => "HTTP/2.0",
            HttpProtocolVersion::Http30 => "HTTP/3.0",
        };
        write!(f, "{}", s)
    }
}

pub enum HttpRequest<'a> {
    GET {
        path: &'a str,
        protocol_version: HttpProtocolVersion,
        request_head: &'a str,
    },
    POST {
        path: &'a str,
        protocol_version: HttpProtocolVersion,
        request_head: &'a str,
    },
    HEAD {
        path: &'a str,
        protocol_version: HttpProtocolVersion,
        request_head: &'a str,
    },
}

impl Display for HttpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GET { path, protocol_version, request_head } => {
                write!(f, "GET {} {}\r\n{}\r\n\r\n", path, protocol_version, request_head)
            }
            Self::POST { path, protocol_version, request_head } => {
                write!(f, "POST {} {}\r\n{}\r\n\r\n", path, protocol_version, request_head)
            }
            Self::HEAD { path, protocol_version, request_head } => {
                write!(f, "HEAD {} {}\r\n{}\r\n\r\n", path, protocol_version, request_head)
            }
        }
    }
}
                        
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_protocol_version_display() {
        assert_eq!(HttpProtocolVersion::Http09.to_string(), "HTTP/0.9");
        assert_eq!(HttpProtocolVersion::Http10.to_string(), "HTTP/1.0");
        assert_eq!(HttpProtocolVersion::Http11.to_string(), "HTTP/1.1");
        assert_eq!(HttpProtocolVersion::Http20.to_string(), "HTTP/2.0");
        assert_eq!(HttpProtocolVersion::Http30.to_string(), "HTTP/3.0");
    }

    #[test]
    fn test_http_request_get_display() {
        let req = HttpRequest::GET {
            path: "/index.html",
            protocol_version: HttpProtocolVersion::Http11,
            request_head: "Host: example.com",
        };
        let expected = "GET /index.html HTTP/1.1\r\nHost: example.com\r\n\r\n";
        assert_eq!(req.to_string(), expected);
    }

    #[test]
    fn test_http_request_post_display() {
        let req = HttpRequest::POST {
            path: "/submit",
            protocol_version: HttpProtocolVersion::Http20,
            request_head: "Host: example.com\r\nContent-Type: application/json",
        };
        let expected = "POST /submit HTTP/2.0\r\nHost: example.com\r\nContent-Type: application/json\r\n\r\n";
        assert_eq!(req.to_string(), expected);
    }

    #[test]
    fn test_http_request_head_display() {
        let req = HttpRequest::HEAD {
            path: "/",
            protocol_version: HttpProtocolVersion::Http10,
            request_head: "Host: test.com",
        };
        let expected = "HEAD / HTTP/1.0\r\nHost: test.com\r\n\r\n";
        assert_eq!(req.to_string(), expected);
    }

    #[test]
    fn test_empty_request_head() {
        let req = HttpRequest::GET {
            path: "/empty",
            protocol_version: HttpProtocolVersion::Http09,
            request_head: "",
        };
        let expected = "GET /empty HTTP/0.9\r\n\r\n\r\n";
        assert_eq!(req.to_string(), expected);
    }
}
