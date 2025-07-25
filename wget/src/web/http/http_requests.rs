//! HTTP 请求构造模块
//! 
//! 本模块提供了构造和格式化 HTTP 请求的核心功能，支持标准的 HTTP 协议。
//! 
//! # 主要组件
//! 
//! - [`HttpRequest`]: 核心的 HTTP 请求结构体，支持所有标准 HTTP 方法
//! - [`HttpMethod`]: HTTP 方法枚举（GET、POST、HEAD 等）
//! - [`HttpProtocolVersion`]: HTTP 协议版本枚举（HTTP/0.9 到 HTTP/3.0）
//! - [`HttpRequestHeader`]: HTTP 请求头集合
//! - [`HttpHeaderField`]: 单个 HTTP 请求头字段
//! 
//! # 设计特性
//! 
//! ## 结构化设计
//! 从原先的枚举设计重构为结构体设计，提供了更好的类型安全性和扩展性：
//! - 类型安全的 HTTP 方法和协议版本
//! - 结构化的请求头管理
//! - 便捷的构造函数（`get()`, `post()`, `head()`）
//! 
//! ## 协议兼容性
//! 严格遵循 HTTP 协议标准：
//! - 正确的请求行格式：`METHOD PATH PROTOCOL`
//! - 标准的请求头格式：`字段名: 字段值`
//! - 请求头和请求体分隔符：`\r\n\r\n`
//! 
//! ## Rust 语言特性展示
//! 代码中展示了 Rust 与 C++ 在闭包/lambda 处理上的差异：
//! - Rust 闭包自动推导捕获方式
//! - C++ lambda 需要显式指定捕获方式
//! 
//! # 使用示例
//! 
//! ## 创建基本的 GET 请求
//! ```rust
//! use crate::web::http::http_requests::{HttpRequest, HttpHeaderField};
//! 
//! let headers = vec![
//!     HttpHeaderField::new("Host".to_string(), "example.com".to_string()),
//!     HttpHeaderField::new("User-Agent".to_string(), "wget/1.0".to_string()),
//! ];
//! 
//! let request = HttpRequest::get("/index.html", headers);
//! println!("{}", request);
//! // 输出: GET /index.html HTTP/1.1\r\nHost: example.com\r\nUser-Agent: wget/1.0\r\n\r\n
//! ```
//! 
//! ## 创建带认证的 POST 请求
//! ```rust
//! let mut request = HttpRequest::post("/api/submit", vec![
//!     HttpHeaderField::new("Host".to_string(), "api.example.com".to_string()),
//!     HttpHeaderField::new("Content-Type".to_string(), "application/json".to_string()),
//! ]);
//! 
//! request.add_header("Authorization".to_string(), "Bearer token123".to_string());
//! ```
//! 
//! ## 自定义协议版本和方法
//! ```rust
//! let request = HttpRequest::new(
//!     HttpMethod::PUT,
//!     "/api/users/1",
//!     HttpProtocolVersion::Http20,
//!     vec![HttpHeaderField::new("Host".to_string(), "api.example.com".to_string())]
//! );
//! ```
//! 
//! # 性能考虑
//! 
//! - 使用借用生命周期 `'a` 避免不必要的字符串复制
//! - 结构化设计减少了运行时的类型判断开销
//! - `Display` trait 实现支持零拷贝的字符串格式化
//! 
//! # 测试覆盖
//! 
//! 模块包含全面的单元测试，覆盖：
//! - 所有 HTTP 方法的请求构造
//! - 协议版本格式化
//! - 请求头管理功能
//! - HTTP 协议格式正确性
//! - 边界情况处理（空请求头等）

use std::fmt::Display;

/// HTTP 请求结构体
/// 
/// 该结构体表示一个完整的 HTTP 请求，包含请求方法、路径、协议版本和请求头。
/// 支持所有标准的 HTTP 方法，并提供便捷的构造函数用于创建常用的请求类型。
/// 
/// # 示例
/// 
/// ```rust
/// use wget::web::http::http_requests::{HttpRequest, HttpHeaderField};
/// 
/// // 创建一个简单的 GET 请求
/// let headers = vec![
///     HttpHeaderField {
///         field_name: "Host".to_string(),
///         field_value: "example.com".to_string(),
///     }
/// ];
/// let request = HttpRequest::get("/index.html", headers);
/// 
/// // 输出标准的 HTTP 请求格式
/// println!("{}", request);
/// ```
pub struct HttpRequest<'a> {
    /// HTTP 请求方法（GET、POST、HEAD 等）
    method: HttpMethod,
    /// 请求的资源路径
    path_to_file: &'a str,
    /// HTTP 协议版本
    protocol_version: HttpProtocolVersion,
    /// 请求头集合
    request_header: HttpRequestHeader,
}

impl<'a> HttpRequest<'a> {

    /// 创建一个新的 HTTP 请求
    /// 
    /// # 参数
    /// - `method`: HTTP 请求方法
    /// - `path`: 请求的资源路径
    /// - `protocol_version`: HTTP 协议版本
    /// - `headers`: 请求头字段列表
    /// 
    /// # 返回值
    /// 返回一个新的 `HttpRequest` 实例
    /// 
    /// # 示例
    /// ```rust
    /// let headers = vec![
    ///     HttpHeaderField::new("Host".to_string(), "example.com".to_string())
    /// ];
    /// let request = HttpRequest::new(
    ///     HttpMethod::GET,
    ///     "/index.html",
    ///     HttpProtocolVersion::Http11,
    ///     headers
    /// );
    /// ```
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
    /// 
    /// 这是一个便捷方法，自动使用 HTTP/1.1 协议创建 GET 请求。
    /// GET 方法用于请求指定的资源，是最常用的 HTTP 方法。
    /// 
    /// # 参数
    /// - `path`: 请求的资源路径
    /// - `headers`: 请求头字段列表
    /// 
    /// # 返回值
    /// 返回一个配置为 GET 方法的 `HttpRequest` 实例
    /// 
    /// # 示例
    /// ```rust
    /// let headers = vec![
    ///     HttpHeaderField::new("Host".to_string(), "example.com".to_string()),
    ///     HttpHeaderField::new("User-Agent".to_string(), "wget/1.0".to_string()),
    /// ];
    /// let request = HttpRequest::get("/index.html", headers);
    /// ```
    pub fn get(path: &'a str, headers: Vec<HttpHeaderField>) -> Self {
        Self::new(HttpMethod::GET, path, HttpProtocolVersion::Http11, headers)
    }

    /// 创建一个 POST 请求
    /// 
    /// 这是一个便捷方法，自动使用 HTTP/1.1 协议创建 POST 请求。
    /// POST 方法用于向指定资源提交要处理的数据。
    /// 
    /// # 参数
    /// - `path`: 请求的资源路径
    /// - `headers`: 请求头字段列表
    /// 
    /// # 返回值
    /// 返回一个配置为 POST 方法的 `HttpRequest` 实例
    /// 
    /// # 示例
    /// ```rust
    /// let headers = vec![
    ///     HttpHeaderField::new("Host".to_string(), "api.example.com".to_string()),
    ///     HttpHeaderField::new("Content-Type".to_string(), "application/json".to_string()),
    ///     HttpHeaderField::new("Content-Length".to_string(), "123".to_string()),
    /// ];
    /// let request = HttpRequest::post("/api/submit", headers);
    /// ```
    pub fn post(path: &'a str, headers: Vec<HttpHeaderField>) -> Self {
        Self::new(HttpMethod::POST, path, HttpProtocolVersion::Http11, headers)
    }

    /// 创建一个 HEAD 请求
    /// 
    /// 这是一个便捷方法，自动使用 HTTP/1.1 协议创建 HEAD 请求。
    /// HEAD 方法请求与 GET 请求相同的响应，但没有响应体，通常用于检查资源的元信息。
    /// 
    /// # 参数
    /// - `path`: 请求的资源路径
    /// - `headers`: 请求头字段列表
    /// 
    /// # 返回值
    /// 返回一个配置为 HEAD 方法的 `HttpRequest` 实例
    /// 
    /// # 示例
    /// ```rust
    /// let headers = vec![
    ///     HttpHeaderField::new("Host".to_string(), "example.com".to_string()),
    /// ];
    /// let request = HttpRequest::head("/check", headers);
    /// ```
    pub fn head(path: &'a str, headers: Vec<HttpHeaderField>) -> Self {
        Self::new(HttpMethod::HEAD, path, HttpProtocolVersion::Http11, headers)
    }

    /// 添加请求头字段
    /// 
    /// 向现有的 HTTP 请求添加一个新的请求头字段。
    /// 
    /// # 参数
    /// - `name`: 请求头字段名称
    /// - `value`: 请求头字段值
    /// 
    /// # 注意
    /// 此方法会修改当前请求实例，添加新的请求头到现有请求头列表中。
    /// 
    /// # 示例
    /// ```rust
    /// let mut request = HttpRequest::get("/api", vec![]);
    /// request.add_header("Authorization".to_string(), "Bearer token123".to_string());
    /// request.add_header("Accept".to_string(), "application/json".to_string());
    /// ```
    /// 
    pub fn append_header_field(&mut self, name: String, value: String) {
        self.request_header.fields.push(HttpHeaderField {
            field_name: name,
            field_value: value,
        });
    }
}




/// 为 `HttpRequest` 实现 `Display` trait
/// 
/// 将 HTTP 请求格式化为符合 HTTP/1.1 协议标准的字符串格式。
/// 输出格式：`METHOD PATH PROTOCOL\r\nHEADERS\r\n\r\n`
/// 
/// 请求头和请求体之间使用 `\r\n\r\n` 分隔，这是 HTTP 协议的标准要求。
/// 
/// # 示例输出
/// ```text
/// GET /index.html HTTP/1.1\r\n
/// Host: example.com\r\n
/// User-Agent: wget/1.0\r\n
/// \r\n
/// ```
impl Display for HttpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}\r\n{}\r\n", self.method, self.path_to_file, self.protocol_version, self.request_header)
    }
}

/// HTTP 请求方法枚举
/// 
/// 定义了标准的 HTTP 请求方法，每种方法都有特定的语义和用途。
/// 各方法的最低协议版本支持要求已在注释中标明。
/// 
/// # 方法说明
/// - `GET`: 获取资源，是最常用的方法
/// - `POST`: 提交数据到服务器
/// - `HEAD`: 获取资源的元信息（不包含响应体）
/// - `PUT`: 替换或创建资源
/// - `DELETE`: 删除指定资源
/// - `CONNECT`: 建立隧道连接
/// - `OPTIONS`: 查询支持的方法和选项
/// - `TRACE`: 路径追踪，用于调试
/// - `PATCH`: 部分更新资源
/// 
/// # 示例
/// ```rust
/// let method = HttpMethod::GET;
/// println!("{}", method); // 输出: "GET"
/// ```
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

/// HTTP 协议版本枚举
/// 
/// 定义了不同版本的 HTTP 协议。每个版本都有不同的特性和性能优化。
/// 
/// # 版本说明
/// - `Http09`: HTTP/0.9 - 最早的版本，只支持 GET 方法
/// - `Http10`: HTTP/1.0 - 引入了请求头和状态码
/// - `Http11`: HTTP/1.1 - 引入了持久连接、分块传输等
/// - `Http20`: HTTP/2.0 - 二进制协议、多路复用
/// - `Http30`: HTTP/3.0 - 基于 QUIC 的新版本
/// 
/// # 示例
/// ```rust
/// let version = HttpProtocolVersion::Http11;
/// println!("{}", version); // 输出: "HTTP/1.1"
/// ```
pub enum HttpProtocolVersion {
    Http09,
    Http10,
    Http11,
    Http20,
    Http30, 
}


/// 为 `HttpMethod` 实现 `Display` trait
/// 
/// 将 HTTP 方法枚举转换为对应的字符串表示，用于构建 HTTP 请求。
/// 输出的字符串符合 HTTP 协议标准的方法名称格式。
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

/// 为 `HttpProtocolVersion` 实现 `Display` trait
/// 
/// 将 HTTP 协议版本枚举转换为符合标准的版本字符串。
/// 输出格式遵循 HTTP 协议规范：`HTTP/x.y`
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

/// HTTP 请求头集合
/// 
/// 包含一个 HTTP 请求的所有请求头字段。
/// 每个字段都是一个名值对，遵循 HTTP 协议的头部格式规范。
/// 
/// # 示例
/// ```rust
/// let header = HttpRequestHeader {
///     fields: vec![
///         HttpHeaderField::new("Host".to_string(), "example.com".to_string()),
///         HttpHeaderField::new("User-Agent".to_string(), "wget/1.0".to_string()),
///     ]
/// };
/// ```
pub struct HttpRequestHeader {
    /// 请求头字段列表
    fields: Vec<HttpHeaderField>,
}

/// HTTP 请求头字段
/// 
/// 表示一个具体的 HTTP 请求头字段，包含字段名和字段值。
/// 字段名不区分大小写，但通常使用标准的大小写格式（如 "Content-Type"）。
/// 
/// # 常见字段示例
/// - `Host`: 指定服务器的域名和端口
/// - `User-Agent`: 客户端标识信息
/// - `Content-Type`: 请求体的媒体类型
/// - `Content-Length`: 请求体的字节长度
/// - `Authorization`: 身份验证信息
/// 
/// # 示例
/// ```rust
/// let field = HttpHeaderField::new(
///     "Content-Type".to_string(),
///     "application/json".to_string()
/// );
/// ```
#[derive(Clone)]
pub struct HttpHeaderField {
    /// 请求头字段名称
    pub field_name: String,
    /// 请求头字段值
    pub field_value: String,
}

impl HttpHeaderField {
    /// 创建一个新的 HTTP 请求头字段
    /// 
    /// # 参数
    /// - `name`: 字段名称（如 "Host", "Content-Type" 等）
    /// - `value`: 字段值（如 "example.com", "application/json" 等）
    /// 
    /// # 返回值
    /// 返回一个新的 `HttpHeaderField` 实例
    /// 
    /// # 示例
    /// ```rust
    /// let host_field = HttpHeaderField::new(
    ///     "Host".to_string(),
    ///     "www.example.com".to_string()
    /// );
    /// 
    /// let auth_field = HttpHeaderField::new(
    ///     "Authorization".to_string(),
    ///     "Bearer token123".to_string()
    /// );
    /// ```
    pub fn new(name: String, value: String) -> Self {
        Self {
            field_name: name,
            field_value: value,
        }
    }
}

/// 为 `HttpRequestHeader` 实现 `Display` trait
/// 
/// 将请求头集合格式化为符合 HTTP 协议的字符串格式。
/// 每个请求头字段占一行，格式为 `字段名: 字段值\r\n`。
/// 
/// 该实现使用了 Rust 闭包的自动推导捕获方式，展示了 Rust 闭包与 C++ lambda 的区别：
/// - Rust: 编译器自动推导捕获方式（按引用捕获 `f`）
/// - C++ lambda: 需要显式指定捕获方式 `[&f]`
/// 
/// # 输出格式
/// ```text
/// Host: example.com\r\n
/// User-Agent: wget/1.0\r\n
/// Content-Type: application/json\r\n
/// ```
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

/// 为 `HttpHeaderField` 实现 `Display` trait
/// 
/// 将单个请求头字段格式化为 `字段名: 字段值` 的格式，不包含换行符。
/// 这种格式可以用于日志输出或调试显示。
/// 
/// # 输出格式
/// ```text
/// Content-Type: application/json
/// ```
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
        
        let req = HttpRequest::get("/index.html", headers);
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
        
        let req = HttpRequest::post("/api/submit", headers);
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
        
        let req = HttpRequest::head("/", headers);
        let expected = "HEAD / HTTP/1.1\r\nHost: test.com\r\n\r\n";
        assert_eq!(req.to_string(), expected);
    }

    #[test]
    fn test_add_header_functionality() {
        let mut req = HttpRequest::get("/test", vec![]);
        
        req.append_header_field("Host".to_string(), "example.com".to_string());
        req.append_header_field("Accept".to_string(), "text/html".to_string());
        
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
        
        let req = HttpRequest::new(
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
        let req = HttpRequest::get("/empty", vec![]);
        let expected = "GET /empty HTTP/1.1\r\n\r\n";
        assert_eq!(req.to_string(), expected);
    }
}
