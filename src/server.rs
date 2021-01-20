use crate::threadpool::ThreadPool;
use std::net::{TcpListener, TcpStream};

pub struct TcpServer {
    ip_address: String,
    handler: fn(TcpStream),
}

impl TcpServer {
    pub fn new(ip_address: String, handler: fn(TcpStream)) -> Self {
        TcpServer {
            ip_address,
            handler
        }
    }

    pub fn start(
        &self,
        max_thread_count: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.ip_address)?;

        let server_thread_pool = ThreadPool::new(max_thread_count);

        for stream in listener.incoming() {
            let handler = self.handler;
            let stream = stream?;

            server_thread_pool.execute(move || {
                handler(stream);
            })?;
        }
        Ok(())
    }
}
