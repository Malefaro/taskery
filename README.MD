# Taskery

Taskery is a task tracking system created for learning purposes. (WIP)

## Used technologies 

- actix-web
- diesel
- gRPC(tonic)
- graphql
- docker
- redis
- postgres

## How to test

1. docker-compose up --build
2. go to http://localhost:5050/
3. play

#### Query examples

Creating user and login to retrieve auth token:
```
mutation {
  createUser(user:{email: "some_email@ab.c", password:"1234"}) {id, email}
  login(auth:{email:"some_email@ab.c", password:"1234"}) {token, userId}
}
```
Creating models(you need to provide an authentication token by setting the http header `{"Authorization": "Bearer <token>"})`:
```
mutation {
  createCompany(userId:1, company:{name:"some_company"}) {id, name}
  createProject(project: {name: "some_project", companyId: 1}) {id, name, companyId}
  createBoard(board:{name:"some_board", projectId:1}) {id, name, projectId}
  createBoardColumn(column:{name:"first column", boardId:1}) {id, name, boardId}
  createTask(task:{name:"some task", text:"Lorem", authorId:1, columnId:1}) {id, columnId, name, text, resolved, createdAt, updatedAt, performerId}
  modifyTask(id:1, task:{performerId:1, resolved:true, text:"ipsum"}) {id, columnId, name, text, resolved, createdAt, updatedAt, performerId}
}
```
Query created models: 
```
query {
  companies(userId:1)
    {
        id, 
        name, 
        projects {
        id, 
        name, 
        boards {
            id,
            name, 
            columns {
            id, 
            name, 
            tasks {
                id,
                name, 
                tags {name},
                resolved,
            }
            }
        }
        }
    }  
}
```


## TODO

- Permissions system
- REST wrapper
- Configuration
- More tests
- Logging
- MongoDB option