use daemonize::Daemonize;

use futures::join;

use std::process;
use tokio::fs;
use tokio::net::UnixListener;
use tokio::prelude::*;
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() {
    /* let daemon = Daemonize::new().pid_file("/tmp/server-manager.pid");

    match daemon.start() {
        Ok(_) => println!("sucessfully started"),
        Err(e) => eprintln!("error while starting: {}", e),
    } */

    ctrlc::set_handler(move || {
        let (pid_res, socket_res) = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                join!(
                    fs::remove_file("/tmp/server-manager.pid"),
                    fs::remove_file("/tmp/server-manager.socket")
                )
            });

        if let Err(e) = pid_res {
            eprintln!("failed to delete pid file: {}", e);
        }
        if let Err(e) = socket_res {
            eprintln!("failed to delete the socket: {}", e);
        }

        process::exit(0);
    })
    .unwrap();

    let mut listener = UnixListener::bind("/tmp/server-manager.socket").unwrap();

    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(socket_res) = incoming.next().await {
            match socket_res {
                Ok(socket) => {
                    println!("Accepted connection from {:?}", socket.peer_addr());
                }
                Err(err) => {
                    println!("accept error = {:?}", err);
                }
            }
        }
    };

    server.await;
}
