create table "user" (
    id uuid primary key,
    email text not null,
    role text not null
);


create table post (
    id uuid primary key default gen_random_uuid(),
    author_id uuid not null references "user"(id) on delete cascade,
    title text not null,
    description text not null,
    listing_type text not null references listing_type(id),
    price decimal not null,
    currency text not null references currency(id),
    property_type text not null references property_type(id),
    condominium boolean not null,
    bedrooms int not null,
    bathrooms int not null,
    service_bathroom bool not null,
    service_bedroom bool not null,
    address text not null,
    year int null,
    orientation text references orientation(id),
    floor int not null,
    parking_lots int not null,
    warehouses int not null,
    built_area int not null,
    land_area int not null,
    coordinates point not null
);
