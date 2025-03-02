use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null().unique_key())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Nickname).string().not_null().unique_key())
                    .col(ColumnDef::new(User::CreatedAt).timestamp().extra("DEFAULT CURRENT_TIMESTAMP".to_owned()).not_null())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp().extra("DEFAULT CURRENT_TIMESTAMP".to_owned()).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}
/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Username,
    Password,
    Nickname,
    CreatedAt,
    UpdatedAt,
}
