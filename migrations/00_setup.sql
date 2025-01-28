-- use better uuids
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- update updated_at column on every update
CREATE OR REPLACE FUNCTION set_updated_at()
    RETURNS TRIGGER AS 
$$
BEGIN
  NEW.updated_at = now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION trigger_updated_at(tablename regclass)
    RETURNS VOID AS 
$$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at 
                        BEFORE UPDATE 
                        ON %s 
                        FOR EACH ROW 
                        WHEN (old IS DISTINCT FROM new) 
                    EXECUTE FUNCTION set_updated_at()', tablename);
END;
$$ LANGUAGE plpgsql;
