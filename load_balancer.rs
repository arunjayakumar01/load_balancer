use clap::{Arg, Command};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncBufReadExt, BufReader, BufWriter, AsyncWriteExt, copy_bidirectional};
use tokio::fs::File;
use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::runtime::Builder;

fn main() -> io::Result<()> {
    let matches = Command::new("Load Balancer")
        .version("1.0")
        .author("Example Author")
        .about("Distributes incoming TCP connections across multiple backend servers")
        .arg(Arg::new("port")
             .short('p')
             .long("port")
             .takes_value(true)
             .help("Port to listen on")
             .default_value("8080"))
        .arg(Arg::new("hosts")
             .short('h')
             .long("hosts")
             .takes_value(true)
             .help("Path to the hosts file")
             .default_value("hosts.txt"))
        .arg(Arg::new("workers")
             .short('w')
             .long("workers")
             .takes_value(true)
             .help("Number of worker threads")
             .default_value("4"))
        .get_matches();

    let port = matches.value_of("port").unwrap();
    let host_file = matches.value_of("hosts").unwrap();
    let worker_threads: usize = matches.value_of_t("workers").unwrap_or_else(|e| e.exit());

    let runtime = Builder::new_multi_thread()
        .worker_threads(worker_threads)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        println!("Load balancer service started on port {}", port);

        let hosts = read_hosts(host_file).await?;
        let logger = Logger::new("load_balancer.log").await?;
        let shared_logger = Arc::new(Mutex::new(logger));
        let connections = Arc::new(ConnectionPool::new());
        let mut host_channels = HashMap::new();

        for host in &hosts {
            let (tx, rx) = mpsc::channel(100);
            let host_clone = host.clone();
            let connections_clone = Arc::clone(&connections);
            let logger_clone = Arc::clone(&shared_logger);
            tokio::spawn(async move {
                manage_host_connections(host_clone, connections_clone, rx, logger_clone).await;
            });
            host_channels.insert(host.clone(), tx);
        }

        let mut round_robin_counter = 0;

        while let Ok((socket, _)) = listener.accept().await {
            let host = hosts[round_robin_counter % hosts.len()].clone();
            if let Some(tx) = host_channels.get(&host) {
                let _ = tx.send(socket).await;
            }
            round_robin_counter = (round_robin_counter + 1) % hosts.len();
        }
        Ok(())
    })
}

async fn read_hosts(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut hosts = Vec::new();
    while let Some(line) = lines.next_line().await? {
        hosts.push(line);
    }
    Ok(hosts)
}

async fn manage_host_connections(host: String, connections: Arc<ConnectionPool>, mut rx: mpsc::Receiver<TcpStream>, logger: Arc<Mutex<Logger>>) {
    while let Some(mut stream) = rx.recv().await {
        let mut upstream = match connections.get_connection(&host).await {
            Some(conn) => conn,
            None => TcpStream::connect(&host).await.unwrap()
        };
        let logger_clone = logger.clone();
        tokio::spawn(async move {
            let _ = copy_bidirectional(&mut stream, &mut upstream).await;
            logger_clone.lock().await.log("Handled connection".to_string()).await;
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

    async fn log(&mut self, message: String) {
        if let Err(e) = self.writer.write_all(format!("{}\n", message).as_bytes()).await {
            eprintln!("Failed to write to log file: {}", e);
        }
        self.writer.flush().await.unwrap();
    }
}

struct ConnectionPool {
    pool: Mutex<HashMap<String, TcpStream>>,
}

impl ConnectionPool {
    fn new() -> Self {
        Self {
            pool: Mutex::new(HashMap::new()),
        }
    }

    async fn get_connection(&self, host: &str) -> Option<TcpStream> {
        let mut pool = self.pool.lock().await;
        pool.remove(host)
    }
}
