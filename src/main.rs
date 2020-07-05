#![allow(non_snake_case)]
extern crate ed25519_dalek;
extern crate rand;

mod frankolang;
mod server;
use frankolang::Frankolang;
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
        Frankolang::startFrankolangInterpreter();
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
    let mut message: [u8; 180] = [0; 180];
    message[0] = 0x03; // payto
    message[73] = 0x04; // fee
    message[73+9] = 0x02; // endsig



    // Testing signature
    // Generating the signature is just used to check if the interpreter is working for now
    // It will be removed later

    let mut messageToSign: [u8; 83] = [0; 83];
    for byte in 0..83 {
        messageToSign[byte] = message[byte];
    }

    // This is just generating a signature on the message in order to test the signature checking on the Interpreter
    // This will be removed later
    let mut csprng = rand::rngs::OsRng;
    let keypair: ed25519_dalek::Keypair = ed25519_dalek::Keypair::generate(&mut csprng);
    let signatureObj: ed25519_dalek::Signature = keypair.sign(&messageToSign);
    let signature = signatureObj.to_bytes();
    let publicKey = keypair.public.to_bytes();

    // Add signature and public key to message
    message.rotate_right(97);
    message[0] = 0x01;

    for byte in 1..65 {
        message[byte] = signature[byte-1];
    }
    for byte in 65..97 {
        message[byte] = publicKey[byte-65];
    }

    // Add commands to message in request buffer
    request[0] = 0x12;
    for byte in 1..181 {
        request[byte] = message[byte-1];
    }
    request[181] = 0x0f;


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
    let mut req: [u8; 1048576] = [0; 1048576];
    std::thread::sleep(std::time::Duration::from_secs(5));
    let err = stream.read(&mut req);
    if err.is_err() {
        eprintln!("Could not read request");
        return;
    }


    let reqSplit: Vec<&[u8]> = req.splitn(1, |num| *num == 0x0a).collect(); // 0x0a is the hex code for \n
    println!("Received request: {}", std::str::from_utf8(reqSplit[0]).expect(""));
    match reqSplit[0] {
        b"newBlock" => {
            // Check blocks proof of work
            // Skipping this for now

            // Perform dryrun of code to check syntax and signatures

            // Send the code to the frankolang interpreter

        },
        b"newCodeSection" => {
            // Verify the signature
            // Verify syntax
            // Add to the unexecuted code variable
        },
        b"reqBlock" => {
            // Find what block they are requesting and write it to the stream
        },
        b"reqUnexec" => {
            // Send all unexecuted code (frankolang code)
            // Unexecuted Code is code that has it's signature verified, but is waiting to be added to a block by miners
        },
        &_ => return
    }
}
