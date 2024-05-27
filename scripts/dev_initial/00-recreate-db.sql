-- DEV ONLY - Brute Force DROP DB (for local dev and unit test)
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
usename = 'ludium_user' OR datname = 'ludium_local';
DROP DATABASE IF EXISTS ludium_local;
DROP USER IF EXISTS ludium_user;

-- DEV ONLY - Dev only password (for local dev and unit test).
CREATE USER ludium_user PASSWORD 'dev_only_pwd';
CREATE DATABASE ludium_local owner ludium_user ENCODING = 'UTF-8';
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
