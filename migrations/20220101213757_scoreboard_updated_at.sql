-- add the updated at column
alter table scoreboards add updated_at timestamp not null default current_timestamp;

-- procedure to update the updated at timestamp for when an update happens on the scoreboards table
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON scoreboards
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- procedure to update the updated at timestamp for when an update happens on the teams table
CREATE OR REPLACE FUNCTION trigger_set_timestamp_teams()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE
        scoreboards
    SET
        updated_at = now()
    where
        scoreboard_id = NEW.scoreboard_id;
    return NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON teams
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp_teams();

