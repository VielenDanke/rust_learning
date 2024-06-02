use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel(1024);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let cloned_tx = tx.clone();
        let mut cloned_rx = tx.subscribe();

        tokio::spawn(async move {
            let (read, mut write) = socket.split();

            let mut reader = BufReader::new(read);

            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if let Ok(bytes_read) = result {
                            if bytes_read == 0 {
                                break;
                            }
                            cloned_tx.send((line.clone(), addr)).unwrap();
                            line.clear();
                        } else {
                            break
                        }
                    }
                    result = cloned_rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        if addr != other_addr {
                            write.write_all(&msg.as_bytes()).await.unwrap();

                            line.clear();
                        }
                    }
                }
            }
        });
    }
}
