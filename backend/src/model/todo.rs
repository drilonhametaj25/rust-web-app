use super::db::Db;
use crate::model;
use sqlb::HasFields;
use crate::security::UserCtx;

// Todo types (Tipo Model)
/**
 * FromRow -> allow sqlx to do "magic" when do fetch
 */
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub cid: i64, // creator id
    pub title: String,
    pub status: TodoStatus
}

//#[derive(sqlb::Fields, Default,Debug, Clone)]
#[derive(Default,Debug, Clone)]
pub struct TodoPatch {
    pub cid: Option<i64>, // creator id
    pub title: Option<String>,
    pub status: Option<TodoStatus>
}

// Enum che abbiamo nel DB
#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "todo_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
    Open,
    Close
}
//sqlb::bindable!(TodoStatus); // For query builder
// TodoMac (tipo metodi del model)
pub struct TodoMac;

impl TodoMac {
    const TABLE: &'static str = "todo";
    const COLUMNS: &'static [&'static str] = &["id", "cid", "title", "status"];
}

impl TodoMac {

    pub async fn create(db: &Db,utx: &UserCtx, data: TodoPatch) -> Result<Todo, model::Error> {
        let sql = "INSERT INTO todo (cid, title) VALUES ($1, $2) returning id, cid, title, status";
        let query = sqlx::query_as::<_, Todo>(&sql)
            .bind(123 as i64)
            .bind(data.title.unwrap_or_else(|| "untitled".to_string()));

        // con query builder
        /*let sb = sqlb::insert()
        .table(Self::TABLE)
        .data(data.fields())
        .returning(&["id", "cid", "title", "status"]);
        let todo_sb = sb.fetch_one(db).await?;*/
           
        let todo = query.fetch_one(db).await?;
        Ok(todo)
    }


    pub async fn list(db: &Db,utx: &UserCtx) -> Result<Vec<Todo>, model::Error> {
        let sql = "SELECT * FROM todo ORDER BY id DESC";

        // build the sqlx-query
        let query = sqlx::query_as::<_, Todo>(&sql); // Si specifica il "tipo/interfaccia" del risultato della query
         
        //execute the query
        let todos = query.fetch_all(db).await?;

        Ok(todos)

    }

    pub async fn get(db:&Db, _utx: &UserCtx, id: i64) -> Result<Todo, model::Error> {
        let sql = "select * FROM todo WHERE id = $1";
        let query = sqlx::query_as::<_, Todo>(&sql)
            .bind(id);
        let todo = query.fetch_one(db).await?;
        Ok(todo)
    }
    
    pub async fn update(db:&Db, _utx: &UserCtx, id: i64, data: TodoPatch) -> Result<Vec<Todo>, model::Error> {
        let sql = "UPDATE todo SET title = $1 WHERE id = $2 returning id, title, cid, status";
        let query = sqlx::query_as::<_, Todo>(&sql)
            .bind(data.title)
            .bind(id);
        let todo = query.fetch_all(db).await?;
        Ok(todo)
    }
}



// Test
#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;