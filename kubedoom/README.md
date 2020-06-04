# Kube DOOM // Modified

# Modified
This repo was forked from [storax/kubedoom](https://github.com/storax/kubedoom). Expanding upon their great work, the following additions have been added so far

## Changes
* **Modified to only target pods with the following label key/value pair: "kubedoom=y1"**
* **The metadata attached to the in-game monsters have been reduced**
  
  * Before this modification each monster would be tagged with '\$NAMESPACE/$FULL-POD-NAME', now it's merely just '\$SHORTENED-POD-NAME'
  * This was changed because it was rather hard visually to differentiate between monsters when there are many and all their names overlap. It was **especially** hard during a live demo.

Given the forked code is under the *GNU GPLv3* license, naturally the modifications within this directory are as well.

Currently the latest build is available at `sequoiia/kubedoom:latest`.

## Kill Kubernetes pods using Id's Doom!

The next level of chaos engineering is here! Kill pods inside your Kubernetes
cluster by shooting them in Doom!

This is a fork of the excellent
[gideonred/dockerdoomd](https://github.com/gideonred/dockerdoomd) using a
slightly modified Doom, forked from https://github.com/gideonred/dockerdoom,
which was forked from psdoom.

![DOOM](assets/doom.jpg)

## Usage

Run `storaxdev/kubedoom:0.4.0` locally:

```console
$ docker run -p5901:5900 \
  --net=host \
  -v ~/.kube:/root/.kube \
  --rm -it --name kubedoom \
  storaxdev/kubedoom:0.4.0
```

Now start a VNC viewer and connect to `localhost:5901`. The password is `1234`:
```console
$ vncviewer viewer localhost:5901
```
You should now see DOOM! Now if you want to get the job done quickly enter the
cheat `idspispopd` and walk through the wall on your right. You should be
greeted by your pods as little pink monsters. Press `CTRL` to fire. If the
pistol is not your thing, cheat with `idkfa` and press `5` for a nice surprise.
Pause the game with `ESC`.

### Killing namespaces

Kubedoom now also supports killing namespaces [in case you have too many of
them](https://github.com/storax/kubedoom/issues/5). Simply set the `-mode` flag
to `namespaces`:

```console
$ docker run -p5901:5900 \
  --net=host \
  -v ~/.kube:/root/.kube \
  --rm -it --name kubedoom \
  storaxdev/kubedoom:0.4.0 \
  -mode namespaces
```

### Running Kubedoom inside Kubernetes

See the example in the `/manifest` directory. You can quickly test it using
[kind](https://github.com/kubernetes-sigs/kind). Create a cluster with the
example config from this repository:

```console
$ kind create cluster --config kind-config.yaml
Creating cluster "kind" ...
 ✓ Ensuring node image (kindest/node:v1.18.2) 🖼
 ✓ Preparing nodes 📦 📦
 ✓ Writing configuration 📜
 ✓ Starting control-plane 🕹️
 ✓ Installing CNI 🔌
 ✓ Installing StorageClass 💾
 ✓ Joining worker nodes 🚜
Set kubectl context to "kind-kind"
You can now use your cluster with:

kubectl cluster-info --context kind-kind

Not sure what to do next? 😅 Check out https://kind.sigs.k8s.io/docs/user/quick-start/
```

This will spin up a 2 node cluster inside docker, with port 5900 exposed from
the worker node. Then run kubedoom inside the cluster by applying the manifest
provided in this repository:

```console
$ export KUBECONFIG="$(kind get kubeconfig-path --name="kind")"
$ kubectl apply -f manifest/
namespace/kubedoom created
deployment.apps/kubedoom created
serviceaccount/kubedoom created
clusterrolebinding.rbac.authorization.k8s.io/kubedoom created
```

To connect run:
```console
$ vncviewer viewer localhost:5900
```

Kubedoom requires a service account with permissions to list all pods and delete
them and uses kubectl 1.18.2.
