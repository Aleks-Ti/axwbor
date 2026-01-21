# axwbor

## blog-cli

```bash
# help
cargo run --bin blog-cli -- --help
```

```bash
# register
cargo run --bin blog-cli -- register   --username alice   --email alice@example.com   --password secret123
```

```bash
# list post
cargo run --bin blog-cli -- list   --limit 10   --offset 0
```

```bash
# login
cargo run --bin blog-cli -- login   --username a1lice   --password secret123
```
