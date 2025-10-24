use sea_orm_migration::{prelude::extension::postgres::Type, prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create OAuth provider enum
        manager
            .create_type(
                Type::create()
                    .as_enum(OAuthCredentials::Provider)
                    .values(vec![
                        OAuthProvider::Google,
                        OAuthProvider::GitHub,
                        OAuthProvider::Facebook,
                    ])
                    .to_owned(),
            )
            .await?;

        // Create users table
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(User::Email)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Name).string_len(100).null())
                    .col(ColumnDef::new(User::Image).string_len(2048).null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on email for fast lookups
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-user-email")
                    .table(User::Table)
                    .col(User::Email)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create oauth_credentials table
        manager
            .create_table(
                Table::create()
                    .table(OAuthCredentials::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OAuthCredentials::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OAuthCredentials::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(OAuthCredentials::Provider)
                            .custom(OAuthCredentials::Provider)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OAuthCredentials::ProviderUserId)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OAuthCredentials::AccessToken)
                            .string_len(2048)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OAuthCredentials::RefreshToken)
                            .string_len(2048)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(OAuthCredentials::ExpiresAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(OAuthCredentials::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(OAuthCredentials::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_oauth_user_id")
                            .from(OAuthCredentials::Table, OAuthCredentials::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on user_id and provider for fast lookups
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-oauth-user-provider")
                    .table(OAuthCredentials::Table)
                    .col(OAuthCredentials::UserId)
                    .col(OAuthCredentials::Provider)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop oauth_credentials table
        manager
            .drop_table(Table::drop().table(OAuthCredentials::Table).to_owned())
            .await?;

        // Drop users table
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        // Drop oauth_provider enum
        manager
            .drop_type(Type::drop().name(OAuthCredentials::Provider).to_owned())
            .await?;

        // Drop index on email
        manager
            .drop_index(
                Index::drop()
                    .name("idx-user-email")
                    .table(User::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Email,
    Name,
    Image,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum OAuthProvider {
    #[iden = "google"]
    Google,
    #[iden = "github"]
    GitHub,
    #[iden = "facebook"]
    Facebook,
}

#[derive(DeriveIden)]
enum OAuthCredentials {
    Table,
    Id,
    UserId,
    Provider,
    ProviderUserId,
    AccessToken,
    RefreshToken,
    ExpiresAt,
    CreatedAt,
    UpdatedAt,
}
