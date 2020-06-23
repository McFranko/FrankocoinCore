// Will host a TCP server and bind it to ip_address and handle connections with handler
// All connections are started on new threads
#![allow(non_snake_case)]

mod threadpool;

pub struct TcpServer {
    pub ipAddress: std::string::String,
    pub handler: fn(std::net::TcpStream)
}

impl TcpServer {
    pub fn start(&self, threads: usize) -> String {
        //Bind to IP
        let listener = std::net::TcpListener::bind(&self.ipAddress);
        if !listener.is_ok() {
            return listener.unwrap_err().to_string();
        }

        let serverThreadPool = threadpool::ThreadPool::new(threads);

        //Handle incoming connections
        for stream in listener.unwrap().incoming() {
            let handler = self.handler;

            // Start new thread for each connection
            serverThreadPool.execute(move || {
                (handler)(stream.unwrap());
            });
        }
        return String::from("");
    }
}
