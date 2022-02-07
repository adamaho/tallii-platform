create table teams_players (
  team_player_id serial primary key,
  team_id integer not null references teams(team_id),
  user_id integer not null references users(user_id),
  created_at timestamp with time zone default (now() at time zone 'utc')
);
