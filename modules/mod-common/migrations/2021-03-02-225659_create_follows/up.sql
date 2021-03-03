CREATE TABLE follows (
            user_id          INTEGER NOT NULL,
            crate_id         INTEGER NOT NULL
        );
ALTER TABLE follows ADD PRIMARY KEY (user_id, crate_id);

CREATE INDEX index_follows_user_id ON follows (user_id);

ALTER TABLE follows ADD CONSTRAINT fk_follows_crate_id
                                 FOREIGN KEY (crate_id) REFERENCES crates (id);
ALTER TABLE follows ADD CONSTRAINT fk_follows_user_id
                                 FOREIGN KEY (user_id) REFERENCES users (id);

ALTER TABLE "follows"
    DROP CONSTRAINT "fk_follows_crate_id",
    ADD CONSTRAINT "fk_follows_crate_id" FOREIGN KEY (crate_id) REFERENCES crates(id) ON DELETE CASCADE;

