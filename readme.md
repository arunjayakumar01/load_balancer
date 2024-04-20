# Load Balancer
<p align="center">
  <img src="https://budstudio-assets.s3.ap-south-1.amazonaws.com/load-balancer-blue-icon-256x256-kmuykxnz.png" alt="Load Balancer">
</p>




This Load Balancer distributes incoming traffic evenly across a set of servers using a `round-robin` algorithm.

## Prerequisites

- **Docker**

## Setup

### Clone the Repository


```bash
git clone <repository-url>
```

### Build the Docker Environment

```bash
docker compose build
```

## Deployment

### Create the Build


```bash
docker compose up
```

This  compiles the application. Compiled binaries are located at `target/x86_64-unknown-linux-gnu/release/load_balancer`.

### Running the Load Balancer

1. **Prepare the Hosts File:** Create a `hosts.txt` file containing the addresses of the upstream hosts that will receive traffic. For example:

```text
0.0.0.0:8001
0.0.0.0:8002
```

2. **Make Executable:** Grant execution permissions to the load balancer binary:

```bash
chmod +x load_balancer
```

3. **Start the Load Balancer:** Launch the load balancer with default settings:

```bash
./load_balancer
```

This command will start the load balancer on the default port (8080).

4. **Custom Configuration:** Optionally,  start the load balancer with custom configuration:

```bash
./load_balancer -p 6060 -h /path/to/hosts.txt -w 4
```

### Command-Line Arguments


| Short | Long       | Description               | Default       |
|-------|------------|---------------------------|---------------|
| `-p`  | `--port`   | Port to listen on         | `8080`        |
| `-h`  | `--hosts`  | Path to upstream hosts    | `./hosts.txt` |
| `-w`  | `--workers`| Number of worker threads  | `4`           |


> Build path : `target/x86_64-unknown-linux-gnu/release/load_balancer`