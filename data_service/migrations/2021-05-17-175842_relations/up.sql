-- Your SQL goes here

create table if not exists company_project_relations (
    id serial primary key,
    project_id int not null references projects(id) on delete CASCADE,
    company_id int not null references companies(id) on delete CASCADE,
    unique(project_id, company_id)
);

create table if not exists project_board_relations (
    id serial primary key,
    project_id int not null references projects(id) on delete CASCADE,
    board_id int not null references boards(id) on delete CASCADE,
    unique(project_id, board_id)
);


create table if not exists project_pages_relations (
    id serial primary key,
    project_id int not null references projects(id) on delete CASCADE,
    page_id int not null references pages(id) on delete CASCADE,
    unique(project_id, page_id)
);

create table if not exists board_column_relations (
    id serial primary key,
    board_id int not null references boards(id) on delete CASCADE,
    column_id int not null references board_columns(id) on delete CASCADE,
    unique(board_id, column_id)
);
create table if not exists column_task_relations (
    id serial primary key,
    task_id int not null references tasks(id) on delete CASCADE,
    column_id int not null references board_columns(id) on delete CASCADE,
    unique(task_id, column_id)
);
create table if not exists task_comment_relations (
    id serial primary key,
    task_id int not null references tasks(id) on delete CASCADE,
    comment_id int not null references task_comments(id) on delete CASCADE,
    unique(task_id, comment_id)
);