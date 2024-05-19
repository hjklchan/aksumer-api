## Akasum-API is used to build simply RESTful-API project

### Install SQLx CLI
```
cargo install sqlx-cli
```

### Migration
```
# Create the up/down migration files
sqlx migrate add -r create_xxx_table

sqlx migrate run --database-url=DATABASE_URL
# Rollback
sqlx migrate revert --database-url=DATABASE_URL

# Enable building in "offline mode" with query!()
cargo sqlx prepare
```