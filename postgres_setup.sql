CREATE TABLE servers (
	id serial primary key,
	last_known_ip inet not null,
	steamid64 bigint
);
