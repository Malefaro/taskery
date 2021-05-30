pub mod macros;
use async_graphql::dataloader::DataLoader;

use crate::database::Database;

pub mod loaders {
    use crate::{loader, loader_related};

    use crate::database::Database;
    use crate::models::{
        board::{
            task::{Tag, Task, TaskComment},
            BoardColumn,
        },
        pages::Page,
        Board, Company, Project, User,
    };
    use async_graphql::{dataloader::Loader, FieldError};
    use std::collections::HashMap;
    use std::pin::Pin;

    loader!(UserLoader, User, get_users_by_id_list);
    loader!(CompanyLoader, Company, get_companies_by_id_list);
    loader!(ProjectLoader, Project, get_projects_by_id_list);
    loader!(BoardLoader, Board, get_boards_by_id_list);
    loader!(PageLoader, Page, get_pages_by_id_list);
    loader!(TaskLoader, Task, get_tasks_by_id_list);

    loader_related!(UserCompaniesLoader, Company, get_users_companies);
    loader_related!(CompanyProjectsLoader, Project, get_companies_projects);
    loader_related!(ProjectBoardsLoader, Board, get_projects_boards);
    loader_related!(ProjectPagesLoader, Page, get_projects_pages);
    loader_related!(BoardColumnsLoader, BoardColumn, get_boards_columns);
    loader_related!(ColumnTasksLoader, Task, get_columns_tasks);
    loader_related!(TaskCommentsLoader, TaskComment, get_tasks_comments);
    loader_related!(TaskTagsLoader, Tag, get_tasks_tags);
}
use crate::create_dataloader;
use loaders::*;

create_dataloader!(
    Dataloader,
    (user_loader, UserLoader),
    (company_loader, CompanyLoader),
    (project_loader, ProjectLoader),
    (board_loader, BoardLoader),
    (page_loader, PageLoader),
    (task_loader, TaskLoader),
    (user_companies_loader, UserCompaniesLoader),
    (company_projects_loader, CompanyProjectsLoader),
    (project_boards_loader, ProjectBoardsLoader),
    (project_pages_loader, ProjectPagesLoader),
    (board_columns_loader, BoardColumnsLoader),
    (column_tasks_loader, ColumnTasksLoader),
    (task_comments_loader, TaskCommentsLoader),
    (task_tags_loader, TaskTagsLoader)
);
