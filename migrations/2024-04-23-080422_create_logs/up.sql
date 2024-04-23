-- Your SQL goes here
CREATE TABLE gpt_logs(
    id BIGSERIAL PRIMARY KEY,
    --------
    ur_id BIGINT NOT NULL,
    code VARCHAR NOT NULL,
    output VARCHAR NOT NULL
);