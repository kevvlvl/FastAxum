# Fast Axum with k8s Monitoring

Web REST API using the Axum Rust framework deployable using podman or on k8s with a comprehensive ci-cd and monitoring stack

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

## Using Kubernetes with k3s

### Prerequisite: Install DevOps Tooling

#### Create Namespace

`kubectl apply -f k8s/1-cicd-namespace.yaml`

#### Gitea

Follow helm instructions here https://docs.gitea.com/installation/install-on-kubernetes

_Note_: For the helm install command, refer to the values file: `helm install -f k8s/2-gitea-values.yaml -n cicd gitea oci://docker.gitea.com/charts/gitea`

When installed, Gitea still won't be accessible unless we create a service of type LoadBalancer using the following YAML file:

`kubectl apply -f k8s/3-gitea-svc-lb.yaml`

Now, on the host, Gitea is accessible from http://localhost:8081

We do need an admin account (for our dev purposes), so we need to create one in Gitea using the following command

_Note_: Replace the pod name by the exact pod deployed and running in your k3s cluster:

`kubectl -n cicd exec -it gitea-7fd8649888-8v6hf -- gitea admin user create --username kev --password adminpass --email kev@kevvlvl.local --admin
`

Now we can login in Gitea and start creating repos which will be important for ArgoCD setup

#### Deploy ArgoCD

Follow helm instructions here https://github.com/argoproj/argo-helm/tree/main/charts/argo-cd#installing-the-chart

Expose argocd via a LB service to make it accessible on the host:

`kubectl apply -f k8s/4-argocd-svc-lb.yaml`

Now, on the host, Argocd is accessible from http://localhost:8082

To login, use user admin and password from the command `kubectl -n cicd get secret argocd-initial-admin-secret -o jsonpath="{.data.password}" | base64 -d
`

Once logged in, you can change the account password. For this exercise, I chose adminpass just like gitea

#### Prepare environments

Before we deploy apps, we need to create a few namespaces for the different concepts that will be deployed: `kubectl apply -f k8s/5-namespaces.yaml`

We will create a namespace for the monitoring stack, for the cert-manager, and for the dev environment.

#### Gitea - GitOps

In Gitea:
Create an organization `devops`. Then, create a repo called `argocd`

In this repo, we will host the files found in k8s/argocd-gitops which are ArgoCD manifests for deployments. 

1. Connect the gitea repository using the load balancer service address `http://gitea-http-lb.cicd.svc.cluster.local:8081/devops/argocd.git`
2. Connect the helm repositories listed in k8s/argocd-gitops (see field spec.source.repoURL)

Then, create a repo called `crds`

In this repo, we will host the files found in k8s/argocd-crds which are operator CRDs for deployments

Once this is done, create an organization `dev`. Then, create a repo called `api`

In this repo, we will host the app deployment in k8s/argocd-app

Once you did this, open ArgoCD and create one ArgoCD deployments manually: one pointing to `argocd-gitops` (app of apps). This will ensure an app-of-apps pattern where ArgoCD automatically deploys any app we desire, simplifying the following tools required for deployment.