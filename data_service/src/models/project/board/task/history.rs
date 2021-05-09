// TODO

// use juniper::GraphQLObject;
// use serde::{Deserialize, Serialize};
// use diesel::{Queryable, Identifiable, Associations};

// use crate::project::board::task::Task;
// use crate::schema::*;

// #[derive(GraphQLObject, Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
// #[belongs_to(Task, foreign_key="task_id")]
// #[table_name="task_histories"]
// pub struct TaskHistory {
//     pub id: i32,
//     pub name: String,
//     pub task_id: i32,
// }
