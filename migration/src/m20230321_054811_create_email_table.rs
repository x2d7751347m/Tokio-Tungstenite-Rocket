/**
 * 📕 EmailStore
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/emailstore
 */
use sea_orm_migration::prelude::*;

use super::m20220120_000001_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Email::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Email::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Email::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-email-user_id")
                            .from(Email::Table, Email::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(Email::Email).string().not_null())
                    .col(
                        ColumnDef::new(Email::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Email::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Email::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Email {
    Table,
    Id,
    UserId,
    Email,
    CreatedAt,
    UpdatedAt,
}
