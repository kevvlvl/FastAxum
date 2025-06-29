# Fast Axum

Web REST API using the Axum Rust framework

## Endpoints

First! Start Jaeger:

```shell
> podman run -d -p4317:4317 -p16686:16686 jaegertracing/all-in-one:latest
```

Jaeger is now reachable at `http://localhost:16686`

Start the app

```shell
> cargo run
```

```shell
> curl localhost:3000
Healthy!
```

```shell
> curl -X POST localhost:3000/domains -H 'Content-Type: application/json' -d '{"name": "Finance", "desc": "The world of Financing"}'
```

## TODO

- Build image using build packs: https://github.com/paketo-community/rust
- Deploy OTEL k8s operator
- Deploy Grafana OSS k8s operator
- Deploy app
