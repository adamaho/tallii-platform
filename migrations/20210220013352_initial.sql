create table users (
    user_id serial primary key,
    username varchar(256) not null,
    email varchar(256) not null unique,
    password varchar(256) not null,
    created_at timestamp not null default current_timestamp
);


