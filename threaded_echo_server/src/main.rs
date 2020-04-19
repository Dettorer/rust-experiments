use std::io::prelude::*;
use std::net::TcpListener;
use std::thread;

const READ_SIZE: usize = 512;

/// Reads incomming data from a stream writes it back to it.
fn echo<T: Read + Write>(mut stream: T) {
    let mut buffer = [0; READ_SIZE];
    loop {
        let size = stream
            .read(&mut buffer)
            .expect("Could not read data from client");
        if size == 0 {
            println!("Client disconnected");
            break;
        }
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
fn handle_client<T>(stream: T)
where
    T: Read + Write + Send + 'static,
{
    thread::spawn(|| echo(stream));
}

fn main() -> std::io::Result<()> {
    better_panic::install();
    let mut args = std::env::args();
    if args.len() != 2 {
        eprintln!("usage: {} <port>", args.next().unwrap());
        std::process::exit(1);
    }
    let port = args.skip(1).next().unwrap();

    let listener = TcpListener::bind(format!("::1:{}", port))
        .expect(&format!("Error binding on port {}", port));

    loop {
        println!("Waiting new clients");
        let (client, _) = listener.accept().expect("Error accepting a new client");

        handle_client(client);
    }
}
