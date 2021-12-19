create table users (
    user_id serial primary key,
    username varchar(256) not null,
    email varchar(256) not null unique,
    password varchar(256) not null,
    created_at timestamp not null default current_timestamp
);

create table games (
    game_id serial primary key,
    name text not null,
    created_at timestamp not null default current_timestamp
);

create table scoreboards (
    scoreboard_id serial primary key,
    name text not null,
    game integer not null references games(game_id),
    created_at timestamp not null default current_timestamp
);

create table teams (
    team_id serial primary key,
    scoreboard_id not null references scoreboards(scoreboard_id) on delete cascade
    name text not null,
    created_at timestamp not null default current_timestamp,
    unique(team_id, scoreboard_id)
);



