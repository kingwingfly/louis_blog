use super::entity::user;
use migration::{Migrator, MigratorTrait as _};
use sea_orm::{
    ActiveValue::Set, ColumnTrait as _, Database, DatabaseConnection, EntityTrait as _, QueryFilter,
};
use tracing::{info, instrument};

use super::error::Result;

#[derive(Debug, Clone)]
pub struct Db {
    inner: DatabaseConnection,
}

#[instrument(fields(database_url = database_url.as_ref()))]
pub async fn establish_connection(database_url: impl AsRef<str>) -> Result<DatabaseConnection> {
    info!("Connecting to database");
    let connection = Database::connect(database_url.as_ref()).await?;
    #[cfg(debug_assertions)]
    Migrator::refresh(&connection).await?;
    #[cfg(not(debug_assertions))]
    Migrator::up(&connection, None).await?;
    info!("Connected to database");
    Ok(connection)
}

impl Db {
    pub async fn new(database_url: impl AsRef<str>) -> Result<Self> {
        Ok(Self {
            inner: establish_connection(database_url).await?,
        })
    }

    pub async fn register(
        &self,
        name: String,
        email: String,
        secret: &[u8],
        salt: &[u8],
        token_salt: &[u8],
    ) -> Result<i32> {
        let id = user::Entity::insert(user::ActiveModel {
            name: Set(name),
            email: Set(email),
            salt: Set(salt.to_vec()),
            secret: Set(secret.to_vec()),
            token_salt: Set(token_salt.to_vec()),
            ..Default::default()
        })
        .exec(&self.inner)
        .await?
        .last_insert_id;
        Ok(id)
    }

    pub async fn get_by_email(&self, email: String) -> Result<Option<user::Model>> {
        Ok(user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.inner)
            .await?)
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<user::Model>> {
        Ok(user::Entity::find_by_id(id).one(&self.inner).await?)
    }
}
