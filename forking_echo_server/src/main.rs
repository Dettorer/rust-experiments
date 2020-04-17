use fork::{fork, Fork};
use std::io::prelude::*;
use std::net::TcpListener;

const READ_SIZE: usize = 512;

/// Reads incomming data from a stream writes it back to it.
fn echo<T>(mut stream: T)
where
    T: Read + Write,
{
    let mut buffer = [0; READ_SIZE];
    loop {
        let size = match stream.read(&mut buffer) {
            Ok(size) if size > 0 => size,
            Ok(_size) => {
                println!("Client disconnected");
                break;
            }
            Err(ref e) => {
                eprintln!("Could not read data from client: {}", e);
                break;
            }
        };
        match stream.write_all(&buffer[0..size]) {
            Ok(()) => (),
            Err(ref e) if e.kind() == std::io::ErrorKind::BrokenPipe => {
                eprintln!("Client connexion lost unexpectedly");
                break;
            }
            Err(ref e) => eprintln!("Could not send data to client: {}, dropping", e),
        }
    }
}

/// Handles a new client in a new (forked) process.
///
/// # Arguments
///
/// * `stream` - The stream to use when communicating with the client
/// * `server` - The server resources, it is dropped by the child process and given back by the
/// parent. Typically, if the client was accepted through a `TcpListener`, this is the listner.
fn handle_client<T, U>(stream: T, listener: U) -> U
where
    T: Read + Write,
{
    match fork() {
        Ok(Fork::Child) => {
            std::mem::drop(listener);
            echo(stream);
            std::process::exit(0);
        }
        Err(e) => eprintln!("Could not fork before handling client: {}", e),
        Ok(Fork::Parent(child_pid)) => println!("Handling a new client in process {}", child_pid),
    }

    listener
}

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    if args.len() != 2 {
        eprintln!("usage: {} <port>", args.next().unwrap());
        std::process::exit(1);
    }
    let port = args.skip(1).next().unwrap();

    let mut listener = match TcpListener::bind(format!("::1:{}", port)) {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Error binding on port {}: {}", port, e);
            std::process::exit(1);
        }
    };

    loop {
        println!("Waiting new clients");
        let client = match listener.accept() {
            Ok((stream, _addr)) => stream,
            Err(e) => {
                eprintln!("Error accepting a new client: {}", e);
                break;
            }
        };

        listener = handle_client(client, listener);
    }

    Ok(())
}
