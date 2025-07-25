use std::fmt::Display;

pub struct HttpRequestNew<'a> {
    method: HttpMethod,
    path_to_file: &'a str,
    protocol_version: HttpProtocolVersion,
    request_header: HttpRequestHeader,
}

impl<'a> HttpRequestNew<'a> {
    /// 创建一个新的 HTTP 请求
    pub fn new(
        method: HttpMethod,
        path: &'a str,
        protocol_version: HttpProtocolVersion,
        headers: Vec<HttpHeaderField>,
    ) -> Self {
        Self {
            method,
            path_to_file: path,
            protocol_version,
            request_header: HttpRequestHeader { fields: headers },
        }
    }

    /// 创建一个 GET 请求
    pub fn get(path: &'a str, headers: Vec<HttpHeaderField>) -> Self {
        Self::new(HttpMethod::GET, path, HttpProtocolVersion::Http11, headers)
    }

    /// 创建一个 POST 请求
    pub fn post(path: &'a str, headers: Vec<HttpHeaderField>) -> Self {
        Self::new(HttpMethod::POST, path, HttpProtocolVersion::Http11, headers)
    }

    /// 创建一个 HEAD 请求
    pub fn head(path: &'a str, headers: Vec<HttpHeaderField>) -> Self {
        Self::new(HttpMethod::HEAD, path, HttpProtocolVersion::Http11, headers)
    }

    /// 添加请求头
    // TODO: 重命名函数名称为 append_header_field()
    pub fn add_header(&mut self, name: String, value: String) {
        self.request_header.fields.push(HttpHeaderField {
            field_name: name,
            field_value: value,
        });
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


impl Display for HttpRequestNew<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}\r\n{}\r\n", self.method, self.path_to_file, self.protocol_version, self.request_header)
    }
}

pub enum HttpMethod {
    /// GET 方法用于请求指定资源（最低支持 HTTP/0.9）
    GET,
    /// POST 方法向指定资源提交要处理的数据（最低支持 HTTP/1.0）
    POST,
    /// HEAD 方法请求与 GET 请求相同的响应，但没有响应体（最低支持 HTTP/1.0）
    HEAD,
    /// PUT 方法用请求负载替换目标资源的所有当前表示（最低支持 HTTP/1.1）
    PUT,
    /// DELETE 方法删除指定资源（最低支持 HTTP/1.1）
    DELETE,
    /// CONNECT 方法建立到目标资源服务器的隧道（最低支持 HTTP/1.1）
    CONNECT,
    /// OPTIONS 方法描述目标资源的通信选项（最低支持 HTTP/1.1）
    OPTIONS,
    /// TRACE 方法沿着到目标资源的路径执行消息回环测试（最低支持 HTTP/1.1）
    TRACE,
    /// PATCH 方法对资源进行部分修改（最低支持 HTTP/1.1）
    PATCH,
}

pub enum HttpProtocolVersion {
    Http09,
    Http10,
    Http11,
    Http20,
    Http30, 
}


impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::CONNECT => "CONNECT",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::TRACE => "TRACE",
            HttpMethod::PATCH => "PATCH",
        };
        write!(f, "{}", s)
    }
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

pub struct HttpRequestHeader {
    fields: Vec<HttpHeaderField>,
}

#[derive(Clone)]
pub struct HttpHeaderField {
    pub field_name: String,
    pub field_value: String,
}

impl HttpHeaderField {
    pub fn new(name: String, value: String) -> Self {
        Self {
            field_name: name,
            field_value: value,
        }
    }
}

impl Display for HttpRequestHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Rust 闭包示例 - 自动推导捕获方式
        self.fields.iter().try_for_each(|field| {
            write!(f, "{}: {}\r\n", field.field_name, field.field_value)
        })
        
        // C++ lambda 等价代码（伪代码注释）：
        // std::for_each(fields.begin(), fields.end(), 
        //     [&f](const auto& field) {  // [&f] 显式指定按引用捕获 f
        //         f << field.field_name << ": " << field.field_value << "\r\n";
        //     });
    }
}

impl Display for HttpHeaderField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}: {}",self.field_name, self.field_value)
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
    fn test_new_http_request_get() {
        let headers = vec![
            HttpHeaderField {
                field_name: "Host".to_string(),
                field_value: "example.com".to_string(),
            },
            HttpHeaderField {
                field_name: "User-Agent".to_string(),
                field_value: "wget/1.0".to_string(),
            },
        ];
        
        let req = HttpRequestNew::get("/index.html", headers);
        // 修正：标准HTTP请求应该以 \r\n\r\n 结尾（请求头和请求体之间的分隔）
        let expected = "GET /index.html HTTP/1.1\r\nHost: example.com\r\nUser-Agent: wget/1.0\r\n\r\n";
        assert_eq!(req.to_string(), expected);
    }

    #[test]
    fn test_new_http_request_post() {
        let headers = vec![
            HttpHeaderField {
                field_name: "Host".to_string(),
                field_value: "example.com".to_string(),
            },
            HttpHeaderField {
                field_name: "Content-Type".to_string(),
                field_value: "application/json".to_string(),
            },
            HttpHeaderField {
                field_name: "Content-Length".to_string(),
                field_value: "123".to_string(),
            },
        ];
        
        let req = HttpRequestNew::post("/api/submit", headers);
        let expected = "POST /api/submit HTTP/1.1\r\nHost: example.com\r\nContent-Type: application/json\r\nContent-Length: 123\r\n\r\n";
        assert_eq!(req.to_string(), expected);
    }

    #[test]
    fn test_new_http_request_head() {
        let headers = vec![
            HttpHeaderField {
                field_name: "Host".to_string(),
                field_value: "test.com".to_string(),
            },
        ];
        
        let req = HttpRequestNew::head("/", headers);
        let expected = "HEAD / HTTP/1.1\r\nHost: test.com\r\n\r\n";
        assert_eq!(req.to_string(), expected);
    }

    #[test]
    fn test_add_header_functionality() {
        let mut req = HttpRequestNew::get("/test", vec![]);
        
        req.add_header("Host".to_string(), "example.com".to_string());
        req.add_header("Accept".to_string(), "text/html".to_string());
        
        let expected = "GET /test HTTP/1.1\r\nHost: example.com\r\nAccept: text/html\r\n\r\n";
        assert_eq!(req.to_string(), expected);
    }

    #[test]
    fn test_custom_http_request() {
        let headers = vec![
            HttpHeaderField {
                field_name: "Authorization".to_string(),
                field_value: "Bearer token123".to_string(),
            },
        ];
        
        let req = HttpRequestNew::new(
            HttpMethod::PUT,
            "/api/users/1",
            HttpProtocolVersion::Http20,
            headers,
        );
        
        let expected = "PUT /api/users/1 HTTP/2.0\r\nAuthorization: Bearer token123\r\n\r\n";
        assert_eq!(req.to_string(), expected);
    }

    #[test]
    fn test_empty_headers() {
        let req = HttpRequestNew::get("/empty", vec![]);
        let expected = "GET /empty HTTP/1.1\r\n\r\n";
        assert_eq!(req.to_string(), expected);
    }
}
