mod threadpool;

use std::net::{TcpStream, TcpListener};

pub struct TcpServer {
    pub ipAddress: std::string::String,
    pub handler: fn(TcpStream)
}

impl TcpServer {
    pub fn start(&self, threads: usize)
        -> Result<(), Box<dyn std::error::Error>>
    {
        let listener = TcpListener::bind(&self.ipAddress)?;

        let serverThreadPool = threadpool::ThreadPool::new(threads);

        for stream in listener.incoming() {
            let handler = self.handler;
            let stream = stream?;

            serverThreadPool.execute(move || {
                handler(stream);
            });
        }
        Ok(())
    }
}
