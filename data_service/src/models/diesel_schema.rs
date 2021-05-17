table! {
    board_column_relations (id) {
        id -> Int4,
        board_id -> Int4,
        column_id -> Int4,
    }
}

table! {
    board_columns (id) {
        id -> Int4,
        name -> Varchar,
        board_id -> Int4,
    }
}

table! {
    boards (id) {
        id -> Int4,
        name -> Varchar,
        project_id -> Int4,
    }
}

table! {
    column_task_relations (id) {
        id -> Int4,
        task_id -> Int4,
        column_id -> Int4,
    }
}

table! {
    companies (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    company_project_relations (id) {
        id -> Int4,
        project_id -> Int4,
        company_id -> Int4,
    }
}

table! {
    company_user_relations (id) {
        id -> Int4,
        user_id -> Int4,
        company_id -> Int4,
    }
}

table! {
    experiences (id) {
        id -> Int4,
        user_id -> Int4,
        company_id -> Int4,
        total_exp -> Nullable<Int4>,
    }
}

table! {
    pages (id) {
        id -> Int4,
        name -> Varchar,
        project_id -> Int4,
        text -> Text,
    }
}

table! {
    project_board_relations (id) {
        id -> Int4,
        project_id -> Int4,
        board_id -> Int4,
    }
}

table! {
    project_pages_relations (id) {
        id -> Int4,
        project_id -> Int4,
        page_id -> Int4,
    }
}

table! {
    projects (id) {
        id -> Int4,
        company_id -> Int4,
        name -> Varchar,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        color -> Varchar,
        exp -> Int4,
        board_id -> Int4,
        author_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    task_comment_relations (id) {
        id -> Int4,
        task_id -> Int4,
        comment_id -> Int4,
    }
}

table! {
    task_comments (id) {
        id -> Int4,
        text -> Text,
        task_id -> Int4,
        author_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    task_tag_relations (id) {
        id -> Int4,
        task_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        name -> Varchar,
        text -> Text,
        resolved -> Bool,
        column_id -> Int4,
        author_id -> Int4,
        performer_id -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        is_admin -> Bool,
    }
}

joinable!(board_column_relations -> board_columns (column_id));
joinable!(board_column_relations -> boards (board_id));
joinable!(board_columns -> boards (board_id));
joinable!(boards -> projects (project_id));
joinable!(column_task_relations -> board_columns (column_id));
joinable!(column_task_relations -> tasks (task_id));
joinable!(company_project_relations -> companies (company_id));
joinable!(company_project_relations -> projects (project_id));
joinable!(company_user_relations -> companies (company_id));
joinable!(company_user_relations -> users (user_id));
joinable!(experiences -> companies (company_id));
joinable!(experiences -> users (user_id));
joinable!(pages -> projects (project_id));
joinable!(project_board_relations -> boards (board_id));
joinable!(project_board_relations -> projects (project_id));
joinable!(project_pages_relations -> pages (page_id));
joinable!(project_pages_relations -> projects (project_id));
joinable!(projects -> companies (company_id));
joinable!(tags -> boards (board_id));
joinable!(tags -> users (author_id));
joinable!(task_comment_relations -> task_comments (comment_id));
joinable!(task_comment_relations -> tasks (task_id));
joinable!(task_comments -> tasks (task_id));
joinable!(task_comments -> users (author_id));
joinable!(task_tag_relations -> tags (tag_id));
joinable!(task_tag_relations -> tasks (task_id));
joinable!(tasks -> board_columns (column_id));

allow_tables_to_appear_in_same_query!(
    board_column_relations,
    board_columns,
    boards,
    column_task_relations,
    companies,
    company_project_relations,
    company_user_relations,
    experiences,
    pages,
    project_board_relations,
    project_pages_relations,
    projects,
    tags,
    task_comment_relations,
    task_comments,
    task_tag_relations,
    tasks,
    users,
);
