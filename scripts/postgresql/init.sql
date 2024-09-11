ALTER USER postgres SET search_path TO data, public;

SELECT 'CREATE DATABASE ramlich' WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'ramlich')\gexec
