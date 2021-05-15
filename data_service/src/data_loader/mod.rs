pub mod companies;
pub mod macros;
pub mod users;
use actix_web::web::Data;
use async_graphql::{dataloader::{Loader, DataLoader}, Context, FieldError};
use std::{any::Any, collections::HashMap, error::Error, pin::Pin};

use crate::{database::Database, loader, models::User};
use crate::{
    database::{postgres::PostgresDB, DatabaseRead},
    models::Company,
};

// pub struct Dataloader(pub Pin<Box<dyn Database + Send + Sync>>);

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


// #[derive(Hash, PartialEq, Eq, Clone, Debug)]
// pub struct UserID(i32);

// #[async_trait::async_trait]
// impl Loader<UserID> for Dataloader {
//     type Value = User;

//     type Error = FieldError;

//     async fn load(&self, keys: &[UserID]) -> Result<HashMap<UserID, Self::Value>, Self::Error> {
//         let res = self
//             .0
//             // .inner
//             .get_users_by_id_list(&keys.iter().cloned().map(|i| i.0).collect::<Vec<i32>>())
//             .await?;
//         let m: HashMap<UserID, Self::Value> = res
//             .into_iter()
//             .map(|model| (UserID(model.id), model))
//             .collect();
//         Ok(m)
//         // todo!()
//     }
// }

// #[derive(Hash, PartialEq, Eq, Clone, Debug)]
// pub struct CompanyID(i32);

// #[async_trait::async_trait]
// impl Loader<CompanyID> for Dataloader {
//     type Value = Company;

//     type Error = FieldError;

//     async fn load(
//         &self,
//         keys: &[CompanyID],
//     ) -> Result<HashMap<CompanyID, Self::Value>, Self::Error> {
//         let res = self
//             .0
//             .get_companies_by_id_list(&keys.iter().cloned().map(|i| i.0).collect::<Vec<i32>>())
//             .await?;
//         let m: HashMap<CompanyID, Self::Value> = res
//             .into_iter()
//             .map(|model| (CompanyID(model.id), model))
//             .collect();
//         Ok(m)
//     }
// }

// #[test]
// fn test() {
//     trait T {}
//     struct A;
//     struct B;
//     impl T for A {}
//     impl T for B {}
//     use std::any::{Any, TypeId};
//     fn get_id<D: Any + Send + Sync>(v: D) -> TypeId {
//         TypeId::of::<D>()
//     }
//     let t1 = Box::pin(A {});
//     let t2 = Box::pin(B {});
//     assert_eq!(get_id(t1), get_id(t2));
//     // pub struct Dataloader(Pin<Box<dyn Database + Send + Sync>>);
//     let db1 = Dataloader(Box::pin(PostgresDB::new("")));
//     // let db2=Dataloader(Box::pin(MongoDB{}));
//     // assert_eq!(get_id(db1), get_id(db2));
// }
