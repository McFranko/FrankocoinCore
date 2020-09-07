// Will host a TCP server and bind it to ip_address and handle connections with handler
// All connections are started on new threads
#![allow(non_snake_case)]

mod threadpool;

pub struct TcpServer
{
    pub ipAddress: std::string::String,
    pub handler: fn(std::net::TcpStream)
}

impl TcpServer
{
    pub fn start(&self, threads: usize)
        -> Result<(), Box<dyn std::error::Error>>
    {
        let listener = std::net::TcpListener::bind(&self.ipAddress)?;

        let serverThreadPool = threadpool::ThreadPool::new(threads);

        for stream in listener.incoming()
        {
            let handler = self.handler;
            let stream = stream?;

            serverThreadPool.execute(move ||
            {
                (handler)(stream);
            });
        }
        Ok(())
    }
}
