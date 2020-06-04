CACHE_DIR=kube-rust-doom-build-cache
IMAGE_NAME="sequoiia/kuberustdoom"
IMAGE_TAG="latest"

set-up-cache:
	mkdir -p $(CACHE_DIR)/src 
	cp -a kube-rust-doom/Cargo.toml $(CACHE_DIR)/Cargo.toml
	cp -a kube-rust-doom/Cargo.lock $(CACHE_DIR)/Cargo.lock
	cp -a main.rs.cache-default $(CACHE_DIR)/src/main.rs

build-cache: set-up-cache
	cd $(CACHE_DIR) && cargo build --release

clean-cache:
	rm -rf $(CACHE_DIR)

build: clean-cache set-up-cache
	docker build -t $(IMAGE_NAME):$(IMAGE_TAG) .