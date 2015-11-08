CREATE TYPE region AS ENUM ('na_west', 'na_east');

CREATE TABLE servers (
	id serial primary key not null unique,
	last_known_ip inet not null,
	last_known_port smallint not null,
	steamid64 bigint,

	is_fallback boolean not null,

	region region not null, 
);

CREATE INDEX ON servers (steamid64);
CREATE INDEX ON servers (last_known_ip) WHERE steamid64 IS NULL;
CREATE INDEX ON servers (is_fallback, region);

CREATE TABLE player_server_relationships (
	id serial primary key not null unique,

	player_steamid64 bigint not null,
	server_id serial references servers(id) not null,
	
	preferred bool not null
);

CREATE TABLE player_preferences (
	steamid64 bigint primary key not null unique
);
