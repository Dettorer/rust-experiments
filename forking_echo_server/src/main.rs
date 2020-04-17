use fork::{fork, Fork};
use std::io::prelude::*;
use std::net::TcpListener;

const READ_SIZE: usize = 512;

/// Reads incomming data from a stream writes it back to it.
fn echo<T: Read + Write>(mut stream: T)
where
    T: Read + Write,
{
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
fn handle_client<T, U>(stream: T, listener: U) -> U
where
    T: Read + Write,
{
    match fork().expect("Could not fork before handling a new client") {
        Fork::Child => {
            std::mem::drop(listener);
            echo(stream);
            std::process::exit(0);
        }
        Fork::Parent(child_pid) => println!("Handling a new client in process {}", child_pid),
    }

    listener
}

fn main() -> std::io::Result<()> {
    better_panic::install();
    let mut args = std::env::args();
    if args.len() != 2 {
        eprintln!("usage: {} <port>", args.next().unwrap());
        std::process::exit(1);
    }
    let port = args.skip(1).next().unwrap();

    let mut listener = TcpListener::bind(format!("::1:{}", port))
        .expect(&format!("Error binding on port {}", port));

    loop {
        println!("Waiting new clients");
        let (client, _) = listener.accept().expect("Error accepting a new client");

        // Under normal circumstances, this function will fork the process, we need to give it
        // ownership of the server socket so that the child (which won't return) can drop it. The
        // parent process will give ownership back to us so that we can accept a new client.
        listener = handle_client(client, listener);
    }
}
