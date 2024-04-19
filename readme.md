
## Load Balancer


### Configure Build Environment 


```bash
git clone <repo>
```

```bash
docker compose build
```

### Create Build

```bash

docker compose up

```

> Build file location `target/x86_64-unknown-linux-gnu/debug/load_balancer`


### RUN 

- create file : hosts.txt , with upstream host details
- Example hosts.txt
```text
0.0.0.0:8001
0.0.0.0:8002
```


```bash
chmod +x load_balancer
```

```bash
./load_balacer
```
> This will run load balancer on PORT 8080

```bash
./load_balacer -p 6060 -h /path/to/hosts.txt -w 4
```

#### Arguments

- -p/--port : PORT 
- -h/--hosts : Path to upstream hosts
- -w/--workers : Thread count
