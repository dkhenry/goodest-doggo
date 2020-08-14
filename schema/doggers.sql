CREATE TABLE IF NOT EXISTS puppers ( id BIGINT(22), name varchar(256), image varchar(256), PRIMARY KEY(id) );
CREATE TABLE IF NOT EXISTS ratings ( id BIGINT(22), user_id BIGINT(22), rating BIGINT, pupper_id BIGINT(22), INDEX (pupper_id));
CREATE TABLE IF NOT EXISTS users ( id BIGINT(22), email varchar(64), password varbinary(256), PRIMARY KEY(id) );
