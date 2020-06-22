// Will host a TCP server and bind it to ip_address and handle connections with handler
// All connections are started on new threads
#![allow(non_snake_case)]

mod threadpool;

pub struct TcpServer {
    pub ipAddress: std::string::String,
    pub handler: fn(std::net::TcpStream)
}

impl TcpServer {
    pub fn start(&self) {
        let listener = std::net::TcpListener::bind(&self.ipAddress);
        if !listener.is_ok() {
            println!("Server failed to bind to ip address: \n{}\nError message:\n{}", &self.ipAddress, listener.unwrap_err());
            std::process::exit(-1);
        }

        for stream in listener.unwrap().incoming() {
            let handler = self.handler;

            // Server is prone to DDOS attacks right now, a thread pool would make sense
            // I'll do that later. Another solution is to just dedicate one memory address
            // to storing the amount of threads available that can be read from and written
            // to by all the threads. I'm not sure if that would be considered memory
            // safe by the compiler though, or even allowed by the kernel.

            std::thread::spawn(move || {
                (handler)(stream.unwrap());
            });
        }
        
    }
}
