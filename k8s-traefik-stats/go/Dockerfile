FROM golang:1-bullseye

WORKDIR /src
COPY ./ ./
RUN go build && mkdir /app && cp k8s-traefik-stats /app/k8s-traefik-stats
RUN chmod +x /app/k8s-traefik-stats

ENTRYPOINT [ "/app/k8s-traefik-stats" ]