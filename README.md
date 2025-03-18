This is a Rust full stack web app built by [leptos](https://github.com/leptos-rs/leptos).

# develop

```sh
cargo install cargo-leptos
cargo install sea-orm-cli

# create a sqlite database abd set DATABASE_URL env var
sea-orm-cli migrate refresh
sea-orm-cli generate entity --expanded-format -o louis_blog/backend/src/db/entity/inner

# start backend
cargo leptos watch
# backend test
cargo run -p http_test
```

# deploy

```sh
# build image
podman build --target runner -t blog .
# run container
podman run -itd -p 3000:3000 --name blog blog
```

[WIP] through git action or k8s
