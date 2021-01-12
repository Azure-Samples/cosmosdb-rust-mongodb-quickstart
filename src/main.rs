use mongodb::sync::Client;
use mongodb::sync::Collection;
use mongodb::bson::{doc, Bson};
use serde::{Deserialize, Serialize};

const INVALID_OP_ERR_MSG: &str = "Invalid operation. use 'create <todo description>', 'list <all/completed/pending>', 'update <todo_id> <all/completed/pending> or 'delete <todo_id>'";

const INVALID_FILTER_ERR_MSG: &str = "Invalid filter. use all, pending or completed";

const CREATE_OPERATION_NAME: &str = "create";
const UPDATE_OPERATION_NAME: &str = "update";
const LIST_OPERATION_NAME: &str = "list";
const DELETE_OPERATION_NAME: &str = "delete";

const TODO_PENDING_STATUS: &str = "pending";
const TODO_COMPLETED_STATUS: &str = "completed";

fn main() {
    let conn_string = std::env::var_os("MONGODB_URL").expect("missing environment variable MONGODB_URL").to_str().expect("failed to get MONGODB_URL").to_owned();
    
    let todos_db_name = std::env::var_os("MONGODB_DATABASE").expect("missing environment variable MONGODB_DATABASE").to_str().expect("failed to get MONGODB_DATABASE").to_owned();

    let todos_collection_name = std::env::var_os("MONGODB_COLLECTION").expect("missing environment variable MONGODB_COLLECTION").to_str().expect("failed to get MONGODB_COLLECTION").to_owned();

    let tm = TodoManager::new(conn_string,todos_db_name.as_str(), todos_collection_name.as_str());
    
    let ops: Vec<String> = std::env::args().collect();
    let op = ops[1].as_str();
    match op {
        CREATE_OPERATION_NAME => tm.add_todo(ops[2].as_str()),
        LIST_OPERATION_NAME => tm.list_todos(ops[2].as_str()),
        UPDATE_OPERATION_NAME => tm.update_todo_status(ops[2].as_str(), ops[3].as_str()),
        DELETE_OPERATION_NAME => tm.delete_todo(ops[2].as_str()),
        _ => panic!(INVALID_OP_ERR_MSG)
    }
}

struct TodoManager {
    coll: Collection
}


impl TodoManager{
    fn new(conn_string: String, db_name: &str, coll_name: &str) -> Self{
        let mongo_client = Client::with_uri_str(&*conn_string).expect("failed to create client");
        let todo_coll = mongo_client.database(db_name).collection(coll_name);
            
        TodoManager{coll: todo_coll}
    }

    fn add_todo(self, desc: &str) {
        let new_todo = Todo {
            todo_id: None,
            desc: String::from(desc),
            status: String::from(TODO_PENDING_STATUS),
        };
    
        let todo_doc = mongodb::bson::to_bson(&new_todo).expect("struct to BSON conversion failed").as_document().expect("BSON to Document conversion failed").to_owned();
        
        let r = self.coll.insert_one(todo_doc, None).expect("failed to add todo");    
        println!("inserted todo with id = {}", r.inserted_id);
    }

    fn list_todos(self, status_filter: &str) {
        let mut filter = doc!{};
        if status_filter == TODO_PENDING_STATUS ||  status_filter == TODO_COMPLETED_STATUS{
            println!("listing '{}' todos",status_filter);
            filter = doc!{"status": status_filter}
        } else if status_filter != "all" {
            panic!(INVALID_FILTER_ERR_MSG)
        }
    
        let mut todos = self.coll.find(filter, None).expect("failed to find todos");
    
        while let Some(result) = todos.next() {
            let todo_doc = result.expect("todo not present");
            let todo: Todo = bson::from_bson(Bson::Document(todo_doc)).expect("BSON to struct conversion failed");
            println!("todo_id: {} | description: {} | status: {}", todo.todo_id.expect("todo id missing"), todo.desc, todo.status);
        }
    }

    fn update_todo_status(self, todo_id: &str, status: &str) {
    
        if status != TODO_COMPLETED_STATUS && status != TODO_PENDING_STATUS {
            panic!(INVALID_FILTER_ERR_MSG)
        }
    
        println!("updating todo_id {} status to {}", todo_id, status);
    
        let id_filter = doc! {"_id": bson::oid::ObjectId::with_string(todo_id).expect("todo_id is not valid ObjectID")};

        let r = self.coll.update_one(id_filter, doc! {"$set": { "status": status }}, None).expect("update failed");
        if r.modified_count == 1 {
            println!("updated status for todo id {}",todo_id);
        } else if r.matched_count == 0 {
            println!("could not update. check todo id {}",todo_id);
        }
    }
    fn delete_todo(self, todo_id: &str) {
        println!("deleting todo {}", todo_id);
    
        let id_filter = doc! {"_id": bson::oid::ObjectId::with_string(todo_id).expect("todo_id is not valid ObjectID")};

        self.coll.delete_one(id_filter, None).expect("delete failed").deleted_count;
    }
}

#[derive(Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    todo_id: Option<bson::oid::ObjectId>,
    #[serde(rename = "description")]
    desc: String,
    status: String,
}