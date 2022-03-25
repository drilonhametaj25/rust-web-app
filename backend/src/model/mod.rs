use thiserror::Error as ThisError;

mod db;
mod todo;

// Enumeratore di errori per poter personalizzare 
#[derive(ThisError, Debug)]
pub enum Error{
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error)
}