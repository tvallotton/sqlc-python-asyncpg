from test.query import models
import dataclasses


@dataclasses.dataclass
class GetPostsAndAuthorsRow:
    post: models.public.Post
    user_: models.public.User
