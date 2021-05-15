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
    companies (id) {
        id -> Int4,
        name -> Varchar,
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
        resolved -> Nullable<Bool>,
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

joinable!(board_columns -> boards (board_id));
joinable!(boards -> projects (project_id));
joinable!(company_user_relations -> companies (company_id));
joinable!(company_user_relations -> users (user_id));
joinable!(experiences -> companies (company_id));
joinable!(experiences -> users (user_id));
joinable!(pages -> projects (project_id));
joinable!(projects -> companies (company_id));
joinable!(tags -> boards (board_id));
joinable!(tags -> users (author_id));
joinable!(task_comments -> tasks (task_id));
joinable!(task_comments -> users (author_id));
joinable!(task_tag_relations -> tags (tag_id));
joinable!(task_tag_relations -> tasks (task_id));
joinable!(tasks -> board_columns (column_id));

allow_tables_to_appear_in_same_query!(
    board_columns,
    boards,
    companies,
    company_user_relations,
    experiences,
    pages,
    projects,
    tags,
    task_comments,
    task_tag_relations,
    tasks,
    users,
);
