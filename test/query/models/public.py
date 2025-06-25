import asyncpg
import dataclasses
import decimal
import shapely
import uuid


@dataclasses.dataclass
class User:
    email: str
    id: uuid.UUID

@dataclasses.dataclass
class Post:
    author_id: uuid.UUID
    id: uuid.UUID
    title: str

@dataclasses.dataclass
class Location:
    foo: list[asyncpg.Range] | None
    location: shapely.Geometry
    money: decimal.Decimal
