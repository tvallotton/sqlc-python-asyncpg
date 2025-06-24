create table user_ (
    id uuid primary key,
    email text not null
);

create table "post" (
    id uuid primary key,
    author_id uuid not null references user_(id),
    title text not null
);

create table "location" (
    foo anyrange[],
    money decimal not null,

    location geometry not null
);

CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');
