# Sqlc-python-asyncpg plugin

## Overview
This plugin is an alternative to the official [sqlc-gen-python](https://github.com/sqlc-dev/sqlc-gen-python) plugin, with added support for:
* sqlc.embed
* Argument grouping
* Type overrides

It is designed for use with the asyncpg driver and offers enhanced customization for generated Python code.

## Usage
An example usage of the plugin with all its options:
```yaml
version: "2"
plugins:
  - name: sqlc-python-asyncpg
    wasm:
      url: https://github.com/tvallotton/sqlc-python-asyncpg/releases/download/v0.1.1/sqlc-python-asyncpg.wasm
      sha256: 896a1602374093a40d9ea97e7693e15adc5da7a7eee01285bc43b249198ffa74

sql:
  - schema: "./migrations"
    queries:
      - app/*/.sql
      - app/*/*/.sql
    engine: postgresql

    codegen:
      - out: app/query
        plugin: sqlc-python-asyncpg
        options:
          package: app.query
          types:
            jsonb:
              python_type: dict
              import: import json
              encode: json.dumps
              decode: json.loads
```

## Options

### Package
The `package` option is required and defines the Python package where generated modules will live. This enables imports between generated modules.
```yaml
options:
  package: myapp.queries
```

### Type overrides

By default, `sqlc-python-asyncpg` uses the native types returned by the `asyncpg` drive. Some of these may be inconvenient or incompatible with your codebase. This plugin lets you override them as needed:

```yaml
options:
  types:
    jsonb: # PostgreSQL type
      python_type: dict
      # If the remapping is handled via a custom codec on
      # the asyncpg connection, the following values can be omitted:
      imports: import json  # optional
      encode: json.dumps    # optional
      decode: json.loads    # optional
```

## Query annotations:
### Namespacing

By default, generated queries are grouped (namespaced) by the SQL filename. For example, queries in author.sql will be accessible like this:
```py
queries.author.my_query(...)
```

You can override the default namespace using the `@namespace:` annotation:

```sql
-- @namespace: author.books
-- name: my_author_book_query :many
```
Which will make it accessible as follows:
```py
queries.author.books.my_author_book_query(...)
```
Namespaces maybe nested arbitrarily.

### Argument grouping
Use the `@group_arguments:` annotation to group SQL query parameters into structured Python classes. Two grouping options are supported:
* `protocol` generates a `typing.Protocol`
* `dataclass` generates a Python `@dataclass`

#### Example
```sql
-- user.sql
-- @group_arguments: protocol
-- name: save
insert into "user" (id, email, role)
values (
    @id,
    @email,
    @role
)
on conflict (id) do update set
    id       = @id,
    email    = @email,
    role     = @role
returning id, email, role;
```

This will generate a protocol which can be used with the `User` model to insert as follows:
```py
user = User(email=email, id=uuid4(), role='admin')
await queries.user.save(user)
```
