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
