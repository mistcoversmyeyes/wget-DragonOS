use crate::log::events::http_events::HttpEvents;

pub trait OnEventHttp {
    fn on_event(&self, event : &HttpEvents);
}


pub trait OnEventFTP {
    // TODO: 定义响应 web模块 中 FTP事件 的接口
}