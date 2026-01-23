# axwbor

## blog-cli

```bash
# help
cargo run --bin blog-cli -- --help
```

```bash
# register
cargo run --bin blog-cli -- register --username alice   --email alice@example.com   --password secret123
```

```bash
# login
cargo run --bin blog-cli -- login --username alice   --password secret123
```

```bash
# create post
cargo run --bin blog-cli -- create --title "super post" --content "super content"
```

```bash
# list post
cargo run --bin blog-cli -- list --limit 10   --offset 0
```

```bash
# update post
cargo run --bin blog-cli -- update --id 1 --title "super post" --content "super content"
```

```bash
# get post
cargo run --bin blog-cli -- get --id 1
```

```bash
# delete post
cargo run --bin blog-cli -- delete --id 1
```
