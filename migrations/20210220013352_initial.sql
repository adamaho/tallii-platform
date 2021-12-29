create table users (
    user_id serial primary key,
    username varchar(256) not null,
    email varchar(256) not null unique,
    password varchar(256) not null,
    created_at timestamp not null default current_timestamp
);

create table scoreboards (
    scoreboard_id serial primary key,
    name text not null,
    game text not null,
    created_by integer not null references users(user_id),
    created_at timestamp not null default current_timestamp
);

create table teams (
    team_id serial primary key,
    scoreboard_id integer not null references scoreboards(scoreboard_id) on delete cascade,
    name text not null,
    score integer not null default 0,
    created_at timestamp not null default current_timestamp,
    unique(team_id, scoreboard_id)
);



