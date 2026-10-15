use std::{eprintln, io::{Read, Write}, net::{TcpListener, TcpStream}, println};

fn main() {
    
    // we are creating TcpListener now by binding it to a ip and port
    let listener = TcpListener::bind("127.0.0.1:7777").unwrap();

    // now we can listen for incomoing TCP connections

    for stream in listener.incoming() {
        
        match stream {
            Ok(stream) => {
                println!("Connection Established");
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Connection Failed with error: {e}")
            }
        }
        
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];

    // loop {
    //     match stream.read(&mut buffer) {
    //         Ok(0) => {
    //             println!("Client Disconnected");
    //             break;
    //         }
    //         Ok(n) => {
    //             println!("read {n} bytes");
                
    //             if let Err(e) = stream.write_all(&buffer[..n]) {
    //                 eprintln!("error in sending {e}");
    //                 break;
    //             } 
    //         }
    //         Err(e) => {
    //             eprintln!("Read error: {e}");
    //             break;
    //         }
    //     }
    // }


    // This works. but it will not keep doing this. It will stop after one round.

    // match stream.read(&mut buffer) {
    //     Ok(n) => {
    //         match stream.write_all(&buffer[..n]) {
    //             Ok(_) => println!("write ok"),
    //             Err(e) => eprintln!("Error {e}"),
    //         }
    //         println!("connection Established!! data -> {}", String::from_utf8_lossy(&buffer));
    //     },
    //     Err(e) => {
    //         eprintln!("Something wrong with data: {e}")
    //     }
    // }


    

    // let mut writer =  stream.try_clone().unwrap();
    // std::io::copy(&mut stream, &mut writer);
}



// next steps. use tokio and get a proper thingy