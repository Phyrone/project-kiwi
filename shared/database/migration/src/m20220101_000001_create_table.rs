use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create()
            .table(Configuration::Table)
            .col(ColumnDef::new(Configuration::Name).string().primary_key().not_null())
            .col(ColumnDef::new(Configuration::Value).json_binary().not_null())
            .to_owned()
        ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .col(ColumnDef::new(Account::Id).big_integer().primary_key())
                    .col(
                        ColumnDef::new(Account::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Account::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Account::SessionSecret)
                            .binary_len(128)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Account::Email).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(AccountKey::Table)
                    .col(ColumnDef::new(AccountKey::Id).big_integer().primary_key())
                    .col(
                        ColumnDef::new(AccountKey::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AccountKey::AccountId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AccountKey::Name).string().not_null())
                    .col(ColumnDef::new(AccountKey::Data).json_binary().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(AccountKey::FkAccountKeyAccountId.to_string())
                            .from(AccountKey::Table, AccountKey::AccountId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Asset::Table)
                    .col(ColumnDef::new(Asset::Id).big_integer().primary_key())
                    .col(
                        ColumnDef::new(Asset::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Asset::Origin).big_integer().not_null())
                    .col(ColumnDef::new(Asset::Public).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Actor::Table)
                    .col(ColumnDef::new(Actor::Id).big_integer().primary_key())
                    .col(
                        ColumnDef::new(Actor::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Actor::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Actor::Name).string().not_null())
                    .col(ColumnDef::new(Actor::Discriminator).small_unsigned().null())
                    .col(ColumnDef::new(Actor::DisplayName).string().null())
                    .col(ColumnDef::new(Actor::Avatar).big_integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(Actor::FkActorAvatarAssetId.to_string())
                            .from(Actor::Table, Actor::Avatar)
                            .to(Asset::Table, Asset::Id),
                    )
                    .col(ColumnDef::new(Actor::Banner).big_integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(Actor::FkActorBannerAssetId.to_string())
                            .from(Actor::Table, Actor::Banner)
                            .to(Asset::Table, Asset::Id),
                    )
                    .index(
                        Index::create()
                            .table(Actor::Table)
                            .col(Actor::Name)
                            .col(Actor::Discriminator)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Asset::Table)
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name(Asset::FkAssetOriginActorId.to_string())
                            .from_tbl(Asset::Table)
                            .from_col(Asset::Origin)
                            .to_tbl(Actor::Table)
                            .to_col(Actor::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Guild::Table)
                    .col(ColumnDef::new(Guild::Id).big_integer().primary_key())
                    .col(
                        ColumnDef::new(Guild::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Guild::OwnerId).big_integer().not_null())
                    .col(ColumnDef::new(Guild::Name).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(Guild::FkGuildOwnerId.to_string())
                            .from(Guild::Table, Guild::OwnerId)
                            .to(Actor::Table, Actor::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(Table::create()
                .table(GuildChannel::Table)
                .col(ColumnDef::new(GuildChannel::Id).big_integer().primary_key())
                .col(ColumnDef::new(GuildChannel::Order).small_unsigned().not_null().default(0))
                .col(ColumnDef::new(GuildChannel::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(GuildChannel::GuildId).big_integer().not_null())
                .col(ColumnDef::new(GuildChannel::ParentId).big_integer().null())
                .col(ColumnDef::new(GuildChannel::Name).string().not_null())
                .col(ColumnDef::new(GuildChannel::Data).json_binary().not_null())
                .foreign_key(ForeignKey::create()
                    .name(GuildChannel::FkGuildChannelGuildId.to_string())
                    .from(GuildChannel::Table, GuildChannel::GuildId)
                    .to(Guild::Table, Guild::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                )
                .to_owned()
            ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .col(ColumnDef::new(Post::Id).big_integer().primary_key())
                    .col(
                        ColumnDef::new(Post::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Post::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Post::AuthorId).big_integer().not_null())
                    .col(ColumnDef::new(Post::Draft).boolean().not_null())
                    .col(ColumnDef::new(Post::Title).string().null())
                    .col(ColumnDef::new(Post::Content).json_binary().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(Post::FkPostAuthorId.to_string())
                            .from(Post::Table, Post::AuthorId)
                            .to(Actor::Table, Actor::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(PostAttatchment::Table)
                    .col(
                        ColumnDef::new(PostAttatchment::PostId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PostAttatchment::AssetId)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(PostAttatchment::PostId)
                            .col(PostAttatchment::AssetId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(PostAttatchment::FkPostAttatchmentPostId.to_string())
                            .from(PostAttatchment::Table, PostAttatchment::PostId)
                            .to(Post::Table, Post::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(PostAttatchment::FkPostAttatchmentAssetId.to_string())
                            .from(PostAttatchment::Table, PostAttatchment::AssetId)
                            .to(Asset::Table, Asset::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(PostAttatchment::Order)
                            .small_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Configuration::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Configuration {
    Table,
    Name,
    Value,
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    SessionSecret,
    Email,
}

#[derive(DeriveIden)]
enum AccountKey {
    Table,
    Id,
    AccountId,
    CreatedAt,
    Name,
    Data,

    FkAccountKeyAccountId,
}

#[derive(DeriveIden)]
enum Asset {
    Table,
    Origin,
    Id,
    CreatedAt,
    Public,
    FkAssetOriginActorId,
}

#[derive(DeriveIden)]
enum Actor {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Name,
    Discriminator,
    Domain,
    DisplayName,

    //Assets
    Avatar,
    Banner,
    FkActorAvatarAssetId,
    FkActorBannerAssetId,
}

#[derive(DeriveIden)]
enum Guild {
    Table,
    Id,
    CreatedAt,
    OwnerId,
    Name,

    FkGuildOwnerId,
}

#[derive(DeriveIden)]
enum GuildChannel {
    Table,
    Id,
    CreatedAt,
    Order,
    GuildId,
    ParentId,
    Name,
    Data,

    FkGuildChannelGuildId,
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    AuthorId,
    Draft,
    Title,
    Content,
    FkPostAuthorId,
}

#[derive(DeriveIden)]
enum PostAttatchment {
    Table,
    Order,
    PostId,
    AssetId,
    FkPostAttatchmentPostId,
    FkPostAttatchmentAssetId,
}
