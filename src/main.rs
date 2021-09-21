use tokio::net::{TcpListener, TcpStream};
use std::env;
use tokio::io;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3);
    let from = String::from(&args[1]);
    let target = Arc::new(String::from(&args[2]));
    let listener = TcpListener::bind(from).await?;
    loop {
        let (socket, addr) = listener.accept().await?;
        let target = Arc::clone(&target);
        tokio::spawn(async move {
            let remote = TcpStream::connect(&*target).await.unwrap();
            let (mut remote_reader, mut remote_writer) = io::split(remote);
            let (mut reader, mut writer) = io::split(socket);
            let remote_handler = tokio::spawn(async move {
                let n = io::copy(&mut remote_reader, &mut writer).await.unwrap_or_else(|e| {
                    println!("remote -> local {}", e);
                    0
                });
                println!("{} read remote {} bytes", addr, n);
            });

            let local_handler = tokio::spawn(async move {
                let n = io::copy(&mut reader, &mut remote_writer).await.unwrap_or_else(|e| {
                    println!("local -> remote {}", e);
                    0
                });
                println!("{} read local {} bytes", addr, n);
            });

            remote_handler.await;
            local_handler.abort();
            // tokio::join!(local_handler,remote_handler);

        });
    }
}