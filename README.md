# Build the server image

```
IMAGE_NAME="gcr.io/rpb-dev/presence-server:$(git rev-parse HEAD)"
docker build -t $IMAGE_NAME .
```

# Push the server image

```
gcloud docker -- push $IMAGE_NAME
```

# Spin up a server with the image

```
gcloud compute instances create-with-container presence-2 \
  --container-image $IMAGE_NAME \
  --machine-type=f1-micro \
  --zone=us-west1-b \
  --tags=allow-grpc
```

I set up a Firewall rule that applies to GCE VMs with the `allow-grpc` tag that allows ingress traffic on port 50051.
