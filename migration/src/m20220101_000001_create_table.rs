use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(integer(User::Id).auto_increment().primary_key())
                    .col(string(User::Name).unique_key())
                    .col(string(User::Email).unique_key())
                    .col(blob(User::Salt))
                    .col(blob(User::TokenSalt))
                    .col(blob(User::Secret))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_user_name_email")
                    .table(User::Table)
                    .col(User::Name)
                    .col(User::Email)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Email,
    Salt,
    TokenSalt,
    Secret,
}
