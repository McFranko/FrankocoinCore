#![allow(non_snake_case)]
extern crate crypto;

mod frankolang;
mod server;
use std::io::Write;
use std::io::Read;

fn main() {
    // I need to make a config file to hold ip addresses and such

    // Check if config exists

    // If config doesn't exist prompt user for the parameters

    // If config does exist then try to interpret it

    // If config syntax is incorrect prompt user if they want to make a new config file
    // If they don't make a new one then exit the program

    // Create server object
    let server = server::TcpServer {
        ipAddress: String::from("localhost:8888"), // Will be a public server (later once config is done)
        handler: connectionHandler
    };
    // Start server on a new thread
    std::thread::spawn(move || {
        println!("{}", server.start(50)); // Prints the error message if there is one
    });



    
    // Start Frankolang interpreter
    std::thread::spawn(|| {
        frankolang::startFrankolangInterpreter();
    });

    std::thread::sleep(std::time::Duration::from_millis(250)); // Waits a bit to ensure the interpreter starts before trying to connect to it
    
    // Connecting to the interpreter
    let frankolangInterpreterErr = std::net::TcpStream::connect("localhost:8354");
    let mut frankolangInterpreter;
    if frankolangInterpreterErr.is_err() {
        eprintln!("Could not connect to Frankolang Interpreter");
        std::process::exit(0);
    } else {
        frankolangInterpreter = frankolangInterpreterErr.unwrap();
    }

    // Testing interpreter /*

    // make request
    let mut request: [u8; 512] = [0; 512];
    request[0] = 0x12;
    request[1] = 0x01;
    request[98] = 0x03;
    request[98+73] = 0x04;
    request[98+73+9] = 0x02;
    request[98+73+9+1] = 0x0f;
    
    // send request
    let err = frankolangInterpreter.write(&request);
    if err.is_err() {
        eprintln!("Cannot write to frankolang interpreter socket");
    }

    // Check if dry run was successful
    let mut res: [u8; 1] = [0];
    let err = frankolangInterpreter.read(&mut res);
    if err.is_err() { eprintln!("Cannot read from frankolang interpreter socket"); }
    if res[0] == 1 {
        println!("Valid frankolang");
        request[0] = 0x11;
        let err = frankolangInterpreter.write(&request);
        if err.is_err() {
            eprintln!("Cannot write to frankolang interpreter socket");
        }
    } else {
        println!("Invalid frankolang");
    }

    //      *\

    // I had it just sleep for a minute now so I can test the server without the program just closing on me
    std::thread::sleep(std::time::Duration::from_secs(60));
}


fn connectionHandler(mut stream: std::net::TcpStream) {
    // Read request
    let mut req: Vec<u8> = vec![];
    std::thread::sleep(std::time::Duration::from_secs(5));
    let err = stream.read(&mut req);
    if err.is_err() {
        eprintln!("Could not read request");
        return;
    }
    // Convert request to string
    let reqStr = String::from_utf8(req).unwrap();

    // I know I could probably make it a little faster by not converting it a string
    // and just comparing the sent buffer against buffers of the keyworks below, but
    // this is just a lot easier. Maybe someone else can do that if they really feel
    // like it.
    let reqSplit: Vec<&str> = reqStr.splitn(1, '\n').collect();
    println!("Received request: {}", reqSplit[0]);
    match reqSplit[0] {
        "newBlock" => {
            // Check blocks proof of work
            // Skipping this for now

            // Verify the signature for each grouping of frankolang
            // Skipping this as well

            // Send the code to the frankolang interpreter
            
        },
        "newCodeSection" => {
            // Verify the signature
            // Verify syntax
            // Add to the unexecuted code variable
        },
        "reqBlock" => {
            // Find what block they are requesting and write it to the stream
        },
        "reqUnexec" => {
            // Send all unexecuted code (frankolang code)
            // Unexecuted Code is code that has it's signature verified, but is waiting to be added to a block by miners
        },
        &_ => return
    }
}