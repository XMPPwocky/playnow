CREATE TABLE servers (
	id serial primary key not null unique,
	last_known_ip inet not null,
	steamid64 bigint 
);

CREATE INDEX ON servers (steamid64);
CREATE INDEX ON servers (last_known_ip);
