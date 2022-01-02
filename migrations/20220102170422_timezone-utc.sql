-- convert scoreboard times to utc
ALTER TABLE scoreboards
  ALTER created_at TYPE timestamptz USING created_at AT TIME ZONE 'UTC',
  ALTER created_at SET DEFAULT now();

ALTER TABLE scoreboards
  ALTER updated_at TYPE timestamptz USING updated_at AT TIME ZONE 'UTC',
  ALTER updated_at SET DEFAULT now();

-- convert team times to utc
ALTER TABLE teams
  ALTER created_at TYPE timestamptz USING created_at AT TIME ZONE 'UTC',
  ALTER created_at SET DEFAULT now();

-- convert user times to utc
ALTER TABLE users
  ALTER created_at TYPE timestamptz USING created_at AT TIME ZONE 'UTC',
  ALTER created_at SET DEFAULT now();