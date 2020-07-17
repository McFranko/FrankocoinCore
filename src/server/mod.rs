// Will host a TCP server and bind it to ip_address and handle connections with handler
// All connections are started on new threads
#![allow(non_snake_case)]

mod threadpool;

pub struct TcpServer {
    pub ipAddress: std::string::String,
    pub handler: fn(std::net::TcpStream)
}

impl TcpServer {
    pub fn start(&self, threads: usize) {
        let listener = match std::net::TcpListener::bind(&self.ipAddress) {
            Ok(listener) => listener,
            Err(error) => { eprintln!("{}", error); return }
        };

        let serverThreadPool = threadpool::ThreadPool::new(threads);

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(stream) => stream,
                Err(error) => { eprintln!("{}", error); return }
            };

            let handler = self.handler;
            serverThreadPool.execute(move || {
                (handler)(stream);
            });
        }
    }
}
