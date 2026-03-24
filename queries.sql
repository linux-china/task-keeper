-- DSN_URL=duckdb::memory:

-- QUERY version
SELECT version();

-- QUERY get_user :one
SELECT * FROM users WHERE id = 1;

-- QUERY extensions-statics
UNPIVOT (SELECT 'community' AS repository, * FROM 'https://community-extensions.duckdb.org/downloads-last-week.json') ON COLUMNS(* EXCLUDE (_last_update, repository)) INTO NAME extension VALUE downloads_last_week ORDER BY downloads_last_week DESC;
