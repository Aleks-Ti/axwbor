# axwbor

## Навигация

- [сервер](#blog-server)
- [cli](#blog-cli)
- [клиент](#blog-client)
- [frontend](#blog-wasm)

## blog-server

Прежде чем приступать к запускам сервера, нобходимо доставить `.env` файл с секретами.
В основном директории лежит файл `.env.example` скопируйте его полностью и удалите постфикс `.example`
Для тестовой проверки будет достаточно данных из example

Предварительно нужно запустить БД:

```bash
make postgres_db
```

или командой, если у вас установлен `Docker`

```bash
docker run --name=blog_db \
            -e SSL_MODE='disable'\
            -e POSTGRES_USER=postgres\
            -e POSTGRES_PASSWORD=postgres\
            -e POSTGRES_DB=blog_db\
            -e TZ=GMT-3\
            -p 5438:5432 -d --rm postgres:17.0-alpine3.19
```

Или вы просто можете создать БД если у вас установлен локально сервер Postgres,
создайте БД с названием БД `blog_db` на порту 5438(скорей всего стандартный у вас уже занят).
Если вы хотите свой порт указать, не забудь изменить данные в `.env` для DATABASE_URL

Запуск сервера:

```bash
# из корня проекта
make start_server
# or
cargo run --bin blog-server
```

## blog-cli

Блог CLI

для проверки работы CLI необходимо запустить сервер [сервер](#blog-server)

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

## blog-client

SDK к API Blog([сервер API](#blog-server))

Библиотека для удоства работы с API Блога

## blog-wasm

Используется `Dioxus`

для запуска работа wasm модуля, необходимо установить `Trunk` -> `cargo install --locked trunk`

Команда для запуска wasm:

```bash
cd blog-wasm
# path/to/blog-wasm$ your trunk command
trunk serve --port 8000
```
