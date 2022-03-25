use thiserror::Error as ThisError;

mod db;
mod todo;

// re-export
pub use db::Db;
pub use db::init_db;
pub use todo::TodoMac;

// Enumeratore di errori per poter personalizzare 
#[derive(ThisError, Debug)]
pub enum Error{
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error)
}