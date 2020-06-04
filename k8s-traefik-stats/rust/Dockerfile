FROM rust:latest as build

WORKDIR /src
COPY . .

RUN mkdir /app
RUN cargo install --root /app --path .

FROM ubuntu:latest
COPY --from=build /app /app

ENTRYPOINT ["/app/bin/k8s-traefik-stats"]