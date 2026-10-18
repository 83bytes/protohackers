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

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client Disconnected");
                break;
            }
            Ok(n) => {
                println!("read {n} bytes");
                
                if let Err(e) = stream.write_all(&buffer[..n]) {
                    eprintln!("error in sending {e}");
                    break;
                } 
            }
            Err(e) => {
                eprintln!("Read error: {e}");
                break;
            }
        }
    }

    // let mut writer =  stream.try_clone().unwrap();
    // std::io::copy(&mut stream, &mut writer);
}

// next steps.
// 1 thread per connection model
// thread pool model
// async with tokio