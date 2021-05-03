-- Your SQL goes here

create table if not exists users(
    id serial primary key,
    email varchar(255) unique not null,
    password varchar(255) not null
);