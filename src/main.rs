use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3);
    let from = String::from(&args[1]);
    let target = Arc::new(String::from(&args[2]));
    let listener = TcpListener::bind(from).await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        let target = Arc::clone(&target);
        tokio::spawn(async move {
            let remote = TcpStream::connect(&*target).await.unwrap();
            let (mut remoteReader, mut remoteWriter) = io::split(remote);
            let (mut reader, mut writer) = io::split(socket);
            tokio::spawn(async move {
                io::copy(&mut remoteReader, &mut writer).await;
            });
            io::copy(&mut reader, &mut remoteWriter).await;
        });
    }
    Ok(())
}