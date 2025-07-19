# Debug 日志输出模块



## 输出基本格式

```bash
DEBUG: 设置为调试模式
DEBUG: 解析URL: http://example.com/file.txt
DEBUG: 正在解析主机 example.com...
DEBUG: IP地址为 93.184.216.34
DEBUG: 正在连接 example.com|93.184.216.34|:80...
DEBUG: 已建立连接
DEBUG: 发送 HTTP 请求:
GET /file.txt HTTP/1.1
Host: example.com
User-Agent: Wget/1.21.1

DEBUG: 收到响应:
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 12345

DEBUG: 开始下载数据...
DEBUG: 已下载 4096 字节...
DEBUG: 已下载 8192 字节...
DEBUG: 下载完成，保存为 file.txt

```