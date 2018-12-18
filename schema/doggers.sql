CREATE TABLE IF NOT EXISTS puppers ( _id BIGINT(22), name varbinary(256), image varbinary(256) );
CREATE TABLE IF NOT EXISTS ratings ( _id BIGINT(22), pupper_name varbinary(256), user_id BIGINT, rating BIGINT ) ;
