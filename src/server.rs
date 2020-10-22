use crate::threadpool::ThreadPool;
use std::net::{TcpListener, TcpStream};

pub struct TcpServer {
    pub ip_address: std::string::String,
    pub handler: fn(TcpStream),
}

impl TcpServer {
    pub fn start(
        &self,
        threads: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.ip_address)?;

        let server_thread_pool = ThreadPool::new(threads);

        for stream in listener.incoming() {
            let handler = self.handler;
            let stream = stream?;

            server_thread_pool.execute(move || {
                handler(stream);
            });
        }
        Ok(())
    }
}
