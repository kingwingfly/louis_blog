This is a Rust full stack web app built by [leptos](https://github.com/leptos-rs/leptos).

# develop

```sh
rustup add rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
cargo leptos watch
```

# deploy

```sh
# build image
podman build --target runner -t blog .
# run container
podman run -itd -p 3000:3000 --name blog blog
```

[WIP] through git action or k8s
