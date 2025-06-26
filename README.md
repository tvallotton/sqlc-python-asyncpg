<<<<<<< HEAD
# Sqlcpy


## Overview
Sqlcpy is an alternative to the official (https://github.com/sqlc-dev/sqlc-gen-python)[sqlc-gen-python] with a couple other features:


## Options


## Type cast
By default all types are returned as they are received by the engine. However,
sometimes it is convenient for sqlc to cast the types automatically. For these cases
you may define some options to configure the generation.

### Example
Suppose we have the following schema
```sql
 CREATE TABLE MyTable (
    info jsonb not null,
    location geometry not null
);
```

By default, `asyncpg` will return `jsonb` and `geometry` types as a json string and a hex string respectively. Hence, the generation will look like this:

```py
@dataclasses.dataclass
class MyTable:
    info: str
    location: str
```

Suppose now, that instead we want the types to be a `dict` and a `shapely.Geometry`. We can define the following type remapping:

```yaml
options:
    types:
        jsonb:
            type: dict
            imports: import json
            cast: json.loads
        geometry:
            type: shapely.Geometry
            import: import shapely
            cast: shapely.from_wkb
```

Now, the generation will look as follows:


```py
import shapely

@dataclasses.dataclass
class Location:
    id: uuid.UUID
    geolocation: shapely.Geometry
```




### Namespaces
=======
# Sqlc-python-asyncpg plugin

## Overview
This plugin is an alternative to the official (https://github.com/sqlc-dev/sqlc-gen-python)[sqlc-gen-python] plugin, with added support for:
* sqlc.embed
* Argument grouping
* Type overrides

It is designed for use with the asyncpg driver and offers enhanced customization for generated Python code.

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
>>>>>>> main
