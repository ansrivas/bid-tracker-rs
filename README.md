bid-tracker-rs
---

[![codecov](https://codecov.io/gh/ansrivas/bid-tracker-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/ansrivas/bid-tracker-rs)
![Linux](https://github.com/ansrivas/bid-tracker-rs/workflows/Linux/badge.svg)

This project implements a bid-tracker trait and concrete implementation with the following functionality:

- record a user’s bid on an item;
- get the current winning bid for an item;
- get all the bids for an item;
- get all the items on which a user has bid;
- build simple REST API to manage bids.
- No persistent store(events are for reporting only). 

#### Why does it exist?
Shows some advanced features of using custom error-handling, mutexed data-being shared among handlers, configuration loading, health-checks etc. in a rust actix-application.

#### Run using docker
```bash
docker build --network=host --build-arg VCS_REF=`git rev-parse --short HEAD` --build-arg BUILD_DATE=`date -u +"%Y-%m-%dT%H:%M:%SZ"`  -t ansrivas/bid-tracker:latest -f Dockerfile .

docker run -p 3000:3000 -i --rm ansrivas/bid-tracker:latest
```

### 

#### Examples:
1. Insert a new bid:
    ```
    curl -H 'Content-Type: application/json' -d '{"useruuid":"ae8f7716-867b-4479-b455-c5769e7475ba", "itemuuid": "b2f9ee6d-79fe-4b14-9c19-35a69a89219a", "timestamp": 1212321, "amount":32}' http://localhost:3000/api/v1/bids | jq
    ```
2. Get all bids on an item:
    ```
    curl -s http://localhost:3000/api/v1/bids/b2f9ee6d-79fe-4b14-9c19-35a69a89219a | jq
    ```
3. Get all bids by a given useruuid:
    ```
    curl -s http://localhost:3000/api/v1/users/ae8f7716-867b-4479-b455-c5769e7475ba/bids | jq
    ```
4. Get the winning bid for a uuid:
    ```
    curl -s http://localhost:3000/api/v1/bids/b2f9ee6d-79fe-4b14-9c19-35a69a89219a/winning | jq
    ```