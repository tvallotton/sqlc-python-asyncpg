
from test.query import models
import asyncpg
import dataclasses
import uuid



@dataclasses.dataclass
class UserQueries:
    connection: asyncpg.Connection

    GET_ALL_USERS = """
        select id, email from "user_"
    """
    GET_USER_BY_ID = """
        select id, email from "user_" where id = $1
    """
    GET_POSTS_AND_AUTHORS = """
        select user_.id, user_.email, post.id, post.author_id, post.title from user_ join post on user_.id = post.author_id
    """
    
    def __init__(self, connection: asyncpg.Connection):
        self.connection = connection

    
    async def get_all_users(self) -> list[models.public.User]:
        rows = await self.connection.fetch(
            self.GET_ALL_USERS
        )
        return [
            models.public.User(
                email=row["email"],
                id=row["id"],
            )
            for row in rows
        ]
    async def get_user_by_id(self, id: uuid.UUID) -> models.public.User | None:
        row = await self.connection.fetchrow(
            self.GET_USER_BY_ID, id
        )
        if row is None:
            return None
        return models.public.User(
            email=row["email"],
            id=row["id"],
        )
    async def get_posts_and_authors(self) -> list[models.user.GetPostsAndAuthorsRow]:
        rows = await self.connection.fetch(
            self.GET_POSTS_AND_AUTHORS
        )
        return [
            models.user.GetPostsAndAuthorsRow(
                post=models.public.Post(
                    author_id=row["author_id"],
                    id=row["id"],
                    title=row["title"],
                ),
                user_=models.public.User(
                    email=row["email"],
                    id=row["id"],
                ),
            )
            for row in rows
        ]

@dataclasses.dataclass
class Queries:
    connection: asyncpg.Connection
    user: UserQueries

    
    def __init__(self, connection: asyncpg.Connection):
        self.connection = connection
        self.user = UserQueries(connection)

    