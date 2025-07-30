# Rust Server

## SeaORM

```bash
sea-orm-cli migrate init
```

migrations/Cargo.tomlを以下のように編集します。

```toml
[dependencies.sea-orm-migration]
version = "1.1.0"
features = [
  # Enable at least one `ASYNC_RUNTIME` and `DATABASE_DRIVER` feature if you want to run migration via CLI.
  # View the list of supported features at https://www.sea-ql.org/SeaORM/docs/install-and-config/database-and-async-runtime.
  # e.g.
  "runtime-tokio-rustls", # `ASYNC_RUNTIME` feature
  "sqlx-mysql",           # `DATABASE_DRIVER` feature
]
```

```bash
sea-orm-cli migrate generate create_table_users
```

```bash
sea-orm-cli migrate \
    -u mysql://user:password@localhost:3306/rsdb
```

```bash
sea-orm-cli generate entity \
    -u mysql://user:password@localhost:3306/rsdb \
    -o src/domain/entities/autogen
```
