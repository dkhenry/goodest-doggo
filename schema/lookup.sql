CREATE TABLE IF NOT EXISTS pupper_seq (id int, next_id bigint, cache bigint, primary key(id)) comment 'vitess_sequence';
INSERT INTO pupper_seq(id, next_id, cache) values(0, 1, 3);
CREATE TABLE IF NOT EXISTS rating_seq (id int, next_id bigint, cache bigint, primary key(id)) comment 'vitess_sequence';
INSERT INTO rating_seq(id, next_id, cache) values(0, 1, 3);
