# Fast Axum

Web REST API using the Axum Rust framework

## Using Podman

Here, we use Jaeger to collect telemetry from the app:

### Create a user-defined network

```shell
> podman network create sandbox
```

### Start Jaeger

```shell
> podman run --network sandbox -d -p4317:4317 -p16686:16686 --name jaeger jaegertracing/all-in-one:latest
```

Jaeger is now reachable at `http://localhost:16686`

### Build and run the app

```shell
> podman image build . -t fastaxum
...
[2/2] STEP 7/7: ENTRYPOINT ["/app/FastAxum"]
[2/2] COMMIT fastaxum
--> 408e2497f01f
Successfully tagged localhost/fastaxum:latest
408e2497f01f8eef27efc818acb93e06a0907937d98c7add256bd83bd14957aa

> podman run --network sandbox -d \
  -e "ADDR=0.0.0.0" \
  -e "PORT=3000" \
  -e "OTEL_EXPORTER_OTLP_ENDPOINT=http://jaeger:4317" \
  -e "OTEL_EXPORTER_OTLP_TRACES_ENDPOINT=http://jaeger:4317" \
  -e "OTEL_EXPORTER_OTLP_METRICS_ENDPOINT=http://jaeger:4317" \
  -e "OTEL_EXPORTER_OTLP_LOGS_ENDPOINT=http://jaeger:4317" \
  --name fastaxum \
  -p3000:3000 \
  fastaxum:latest
```

Now, we can curl the app:

```shell
> curl localhost:3000
"Healthy!"
```

```shell
> curl -X POST localhost:3000/domains -H 'Content-Type: application/json' -d '{"name": "Finance", "desc": "The world of Financing"}'
```

## Using Kubernetes (k3s)

### TODO

- Deploy Gitea
- Deploy ArgoCD
- Deploy OTEL k8s operator
- Deploy Grafana OSS k8s operator
- Deploy Buildah (for image builds)
- Deploy the app using ArgoCD