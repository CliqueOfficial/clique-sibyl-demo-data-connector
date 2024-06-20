# clique-sibyl-demo-data-connector

## Setup

### Environment 

- Ubuntu 20.04 with Linux Kernel ≥ 5.11
- CPU: Intel Xeon E-2288G
- Docker (>= 20.10.21) & Docker-Compose


## Build & Deploy

* Build Sibyl: 

  ```bash
  docker build -t sibyl -f Dockerfile.sibyl .
  ```


* Build DCsv2 Sibyl:

  ```bash
  docker build -t sibyl -f Dockerfile.DCsv2.sibyl .
  ```


* Build DCsv2 custom DCAP service:

  ```bash
  docker build -t pccs -f Dockerfile.DCsv2.pccs .
  ```

* Deploy Sibyl:

  ```bash
  docker compose -f docker-compose.yml up
  ```

* Deploy Sibyl with custom DCAP service:

  ```bash
  docker compose -f docker-compose-dcap.yml up
  ```

Then Sibyl will run and listen on port 3443.

> For Azure VMs, custom DCAP service is only avaiable for DCsv2 and is not supported in DCsv3.

## Test

For example, if you want to make a get query for url `https://api.github.com/repos/github/repo-name`:

```bash
curl -k --location 'https://localhost:3443/query' \
--header 'Content-Type: application/json' \
--data '{
    "query_type": "demo_get",
    "query_param": {
        "host": "api.github.com",
        "url": "/repos/github/repo-name",
        "port": 443
    }
}'
```
