---
name: Architecture
---

# Architecture

We use a microservice monolyth hybrid architecture.

```mermaid
flowchart BT
    subgraph Backend
        minio[(Minio/S3)]
        postgres[(Postgres)]
        keydb[(KeyDB)]
        rethinkdb[(RethinkDB)]
        core[Core]
        gateway[Gateway]
        relay[Relay]
        media[Media]
        sveltekit_backend[/SvelteKit Backend\]
    %% Backend Connections
        media .-> |Tiered Caching| media 
        media --> postgres
        core ---> postgres & keydb & rethinkdb
        sveltekit_backend -->|GraphQL| core
        gateway -->|GraphQL| core 
        relay ----> rethinkdb
        media ----> minio
    end

    svletekit_frontend[\SvelteKit Frontend/]
    svletekit_frontend -->|Html/GraphQL| sveltekit_backend
    svletekit_frontend -->|Realtime Websocket| gateway
    svletekit_frontend -->|RTC calls| relay
    svletekit_frontend ----->|Down/Upload Media| media
    user[User]
    user .- svletekit_frontend
```
