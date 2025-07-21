use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::io::{self, Result, Read, Write};
use std::time::Duration;
use crate::log;
use crate::log::debuglog::events;
use crate::log::debuglog::productor::{DebugLogProductor, OnEventDebug};

pub struct HttpClient {
    pub stream: TcpStream,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub log_productor : DebugLogProductor,
}

impl HttpClient {
    

}

