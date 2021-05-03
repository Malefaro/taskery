-- Your SQL goes here
CREATE EXTENSION moddatetime;

create table if not exists companies(
    id serial primary key,
    name varchar(255) not null
);

create table if not exists company_user_relations (
    id serial primary key,
    user_id int not null references users(id) on delete CASCADE,
    company_id int not null references companies(id) on delete CASCADE,
    unique(user_id, company_id)
);


create table if not exists experiences (
    id serial primary key,
    user_id int not null references users(id) on delete CASCADE,
    company_id int not null references companies(id) on delete CASCADE,
    total_exp int,
    unique(user_id, company_id)
);


create table if not exists projects (
    id serial primary key,
    company_id int not null references companies(id) on delete CASCADE,
    name varchar(255) not null
);

create table if not exists pages (
    id serial primary key,
    name varchar(255) not null,
    project_id int not null references projects(id) on delete CASCADE
);

create table if not exists boards (
    id serial primary key,
    name varchar(255) not null,
    project_id int not null references projects(id) on delete CASCADE
);

create table if not exists board_columns(
    id serial primary key,
    name varchar(255) not null,
    board_id int not null references boards(id) on delete CASCADE
);

create table if not exists tasks(
    id serial primary key,
    name varchar(255) not null,
    text TEXT not null DEFAULT '',
    resolved bool DEFAULT false,
    column_id int not null references board_columns(id) on delete CASCADE,
    author_id int not null references users(id) on delete CASCADE,
    performer_id int not null references users(id) on delete CASCADE,
    created_at TIMESTAMPTZ not null default now(),
    updated_at TIMESTAMPTZ not null default now()
);

CREATE TRIGGER mdt_tasks
  BEFORE UPDATE ON tasks
  FOR EACH ROW
  EXECUTE PROCEDURE moddatetime (updated_at);


create table if not exists tags(
    id serial primary key,
    name varchar(255) not null,
    color varchar(255) not null,
    exp int not null,
    board_id int not null references boards(id) on delete CASCADE,
    author_id int not null references users(id) on delete CASCADE,
    created_at TIMESTAMPTZ not null default now(),
    updated_at TIMESTAMPTZ not null default now()
);

CREATE TRIGGER mdt_tags
  BEFORE UPDATE ON tags
  FOR EACH ROW
  EXECUTE PROCEDURE moddatetime (updated_at);


create table if not exists task_tag_relations (
    id serial primary key,
    task_id int not null references tasks(id) on delete CASCADE,
    tag_id int not null references tags(id) on delete CASCADE,
    unique(tag_id, task_id)
);

create table if not exists task_comments (
    id serial primary key,
    text TEXT not null default '',
    task_id int not null references tasks(id) on delete CASCADE,
    author_id int not null references users(id) on delete CASCADE,
    created_at TIMESTAMPTZ not null default now(),
    updated_at TIMESTAMPTZ not null default now()
);

CREATE TRIGGER mdt_task_comments
  BEFORE UPDATE ON task_comments
  FOR EACH ROW
  EXECUTE PROCEDURE moddatetime (updated_at);