strsim-node.node: Cargo.toml Cargo.lock Makefile src/lib.rs
	docker run \
	--env RUSTFLAGS="-C target-feature=-crt-static" \
	--workdir /app \
	--rm \
	--volume `pwd`:/app rust:alpine \
	/bin/sh -c \
	"apk add --no-cache libc-dev; cargo build --message-format=json-render-diagnostics --release; mv target/release/libstrsim_node.so ./strsim-node.node"
