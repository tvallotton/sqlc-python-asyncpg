-- name: get_all_users :many
select * from "user_";;

-- name: get_user_by_id :one
select * from "user_" where id = @id;

-- name: get_posts_and_authors :many
select sqlc.embed(user_), sqlc.embed(post) from user_ join post on user_.id = post.author_id;
