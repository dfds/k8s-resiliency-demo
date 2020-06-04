# k8s-resiliency-demo

This repository contains multiple projects that was used for a live DevEx demo on the 28th May 2020.

Full description with screenshots coming soon..ish.

# Projects

## K8s-traefik-stats
* Intended to be run as a sidecar pod next to a Traefik deployment
* Provides an HTTP API and websocket on port 3100
* The HTTP API gives total(*since start of server*) http status codes.
* The websocket provides:

  * Every time Traefik logs a request in its access log file, it is immediately broadcast to all websocket listeners.
  * For every second where there is at least one request, a message is broadcast with total http status codes within that one second timespan

---

* Additionally within this directory is also a web sub-directory that contains some rather barebones HTML/JS/CSS files that utilises the aforementioned HTTP API and websocket to visualise the data. 

## Kubedoom

The files within this directory was forked from [storax/kubedoom](https://github.com/storax/kubedoom). Expanding upon their great work, the following additions have been added so far

## Changes
* **Modified to only target pods with the following label key/value pair: "kubedoom=y1"**
* **The metadata attached to the in-game monsters have been reduced**

Kubedoom uses a different license compared to the rest of this repository, namely *GNU GPLv3* license. The full license can be found within the kubedoom directory.

## Kube-rust-doom

Unfinished rewrite of Kubedoom, merely just for the fun of it.