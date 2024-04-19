use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncBufReadExt, BufReader, BufWriter, copy_bidirectional, AsyncWriteExt};
use tokio::fs::File;
use tokio::sync::{Mutex, mpsc};
use std::sync::Arc;
use std::collections::HashMap;

#[tokio::main(flavor = "multi_thread", worker_threads = 100)]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    let hosts = read_hosts("hosts.txt").await?;
    let logger = Logger::new("load_balancer.log").await?;
    let _shared_logger = Arc::new(Mutex::new(logger));

    let mut connections = HashMap::new();

    for host in &hosts {
        let (tx, mut rx) = mpsc::channel(100);
        let host_clone = host.clone();
        tokio::spawn(async move {
            manage_host_connections(host_clone, &mut rx).await;
        });
        connections.insert(host.clone(), tx);
    }

    let mut round_robin_counter = 0;

    loop {
        let (socket, _) = listener.accept().await?;
        let host = hosts[round_robin_counter % hosts.len()].clone();
        if let Some(tx) = connections.get(&host) {
            tx.send(socket).await.unwrap();
        }
        round_robin_counter = (round_robin_counter + 1) % hosts.len();
    }
}

async fn read_hosts(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut hosts = Vec::new();
    while let Some(line) = lines.next_line().await? { // Corrected this line
        hosts.push(line);
    }
    Ok(hosts)
}


async fn manage_host_connections(host: String, rx: &mut mpsc::Receiver<TcpStream>) {
    while let Some(mut stream) = rx.recv().await {
        let mut upstream = TcpStream::connect(&host).await.unwrap();
        tokio::spawn(async move {
            let _ = copy_bidirectional(&mut stream, &mut upstream).await;
        });
    }
}

struct Logger {
    writer: BufWriter<File>,
}

impl Logger {
    async fn new(file_name: &str) -> io::Result<Logger> {
        let file = File::create(file_name).await?;
        Ok(Logger { writer: BufWriter::new(file) })
    }

    #[allow(dead_code)]
    async fn log(&mut self, message: String) {
        if let Err(e) = self.writer.write_all(format!("{}\n", message).as_bytes()).await {
            eprintln!("Failed to write to log file: {}", e);
        }
        self.writer.flush().await.unwrap();
    }
}
