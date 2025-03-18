use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum DbErr {
    #[snafu(context(false), display("Database error: {source}"))]
    ConnectionLost { source: sea_orm::error::DbErr },
}

pub type Result<T> = core::result::Result<T, DbErr>;
