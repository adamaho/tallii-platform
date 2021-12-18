create table users (
    user_id serial primary key,
    email varchar(256) not null unique,
    created_at timestamp not null default current_timestamp
);


