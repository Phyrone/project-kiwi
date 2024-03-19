use enum_iterator::{all, Sequence};
use sea_orm_migration::prelude::*;

use crate::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Asset::Table)
                    .col(
                        ColumnDef::new(Asset::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AssetVariant::Table)
                    .col(
                        ColumnDef::new(AssetVariant::AssetId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssetVariant::Variant)
                            .string_len(64)
                            .not_null(),
                    )
                    .col(ColumnDef::new(AssetVariant::Data).json_binary().not_null())
                    .primary_key(
                        Index::create()
                            .col(AssetVariant::AssetId)
                            .col(AssetVariant::Variant),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(AssetVariant::Table)
                            .to_tbl(Asset::Table)
                            .from_col(AssetVariant::AssetId)
                            .to_col(Asset::Id),
                    )
                    .to_owned(),
            ).await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(
                        ColumnDef::new(User::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string_len(320).not_null())
                    .col(ColumnDef::new(User::Password).text().null())
                    .index(
                        Index::create()
                            .table(User::Table)
                            .col(User::Email)
                            .unique()
                            .nulls_not_distinct(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(UserKey::Table)
                    .col(
                        ColumnDef::new(UserKey::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserKey::UserId).big_unsigned().not_null())
                    .col(
                        ColumnDef::new(UserKey::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserKey::Key).json_binary().not_null())
                    .col(ColumnDef::new(UserKey::Metadata).json().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(UserKey::Table)
                            .to_tbl(User::Table)
                            .from_col(UserKey::UserId)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .col(
                        ColumnDef::new(Profile::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Profile::OwningUserId).big_unsigned().null())
                    .col(ColumnDef::new(Profile::Name).string_len(64).not_null())
                    .col(
                        ColumnDef::new(Profile::Discriminator)
                            .small_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Profile::DisplayName)
                            .string_len(64)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Profile::Picture).big_integer().null())
                    .col(ColumnDef::new(Profile::Banner).big_integer().null())
                    .col(ColumnDef::new(Profile::Metadata).json_binary().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Profile::Table)
                            .to_tbl(User::Table)
                            .from_col(Profile::OwningUserId)
                            .to_col(User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Profile::Table)
                            .to_tbl(Asset::Table)
                            .from_col(Profile::Picture)
                            .to_col(Asset::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Profile::Table)
                            .to_tbl(Asset::Table)
                            .from_col(Profile::Banner)
                            .to_col(Asset::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Guild::Table)
                    .col(
                        ColumnDef::new(Guild::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Guild::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Guild::Name).string_len(128).not_null())
                    .col(ColumnDef::new(Guild::OwnerId).big_unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Guild::Table)
                            .to_tbl(User::Table)
                            .from_col(Guild::OwnerId)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(ChannelType::Table)
                    .values(all::<ChannelType>().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Channel::Table)
                    .col(
                        ColumnDef::new(Channel::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Channel::GuildId).big_unsigned().not_null())
                    .col(ColumnDef::new(Channel::ParentId).big_unsigned().null())
                    .col(
                        ColumnDef::new(Channel::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Channel::Name).string_len(128).not_null())
                    .col(
                        ColumnDef::new(Channel::Type)
                            .custom(ChannelType::Table.into_iden())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Channel::Position).integer().not_null())
                    .col(ColumnDef::new(Channel::Topic).string().null())
                    .col(ColumnDef::new(Channel::Metadata).json().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Channel::Table)
                            .to_tbl(Guild::Table)
                            .from_col(Channel::GuildId)
                            .to_col(Guild::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Channel::Table)
                            .to_tbl(Channel::Table)
                            .from_col(Channel::ParentId)
                            .to_col(Channel::Id),
                    )
                    .index(
                        Index::create()
                            .table(Channel::Table)
                            .col(Channel::GuildId)
                            .col(Channel::ParentId)
                            .col(Channel::Name)
                            .unique()
                            .nulls_not_distinct(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Flags::Table)
                    .values(all::<Flags>().skip(1))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .col(
                        ColumnDef::new(Post::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::ChannelId).big_unsigned().null())
                    .col(ColumnDef::new(Post::AuthorId).big_unsigned().null())
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
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Body).json_binary().null())
                    .col(ColumnDef::new(Post::Metadata).json_binary().null())
                    .col(
                        ColumnDef::new(Post::Tags)
                            .array(ColumnType::String(None))
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Post::Flags)
                            .array(ColumnType::Custom(Flags::Table.into_iden()))
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Post::Table)
                            .to_tbl(Channel::Table)
                            .from_col(Post::ChannelId)
                            .to_col(Channel::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Post::Table)
                            .to_tbl(Profile::Table)
                            .from_col(Post::AuthorId)
                            .to_col(Profile::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PostAttachment::Table)
                    .col(
                        ColumnDef::new(PostAttachment::PostId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PostAttachment::AssetId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PostAttachment::Position)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .primary_key(
                        Index::create()
                            .col(PostAttachment::PostId)
                            .col(PostAttachment::AssetId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(PostAttachment::Table)
                            .to_tbl(Post::Table)
                            .from_col(PostAttachment::PostId)
                            .to_col(Post::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(PostAttachment::Table)
                            .to_tbl(Asset::Table)
                            .from_col(PostAttachment::AssetId)
                            .to_col(Asset::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(PostAttachment::Table).to_owned()).await?;

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;

        manager.drop_type(Type::drop().name(Flags::Table).to_owned()).await?;

        manager
            .drop_table(Table::drop().table(Channel::Table).to_owned())
            .await?;

        manager.drop_type(Type::drop().name(ChannelType::Table).to_owned()).await?;

        manager
            .drop_table(Table::drop().table(Guild::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Profile::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserKey::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Asset::Table).to_owned())
            .await?;


        Ok(())
    }
}

#[derive(DeriveIden)]
enum Asset {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum AssetVariant {
    Table,
    AssetId,
    Variant,
    Data,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Email,
    Password,
}

#[derive(DeriveIden)]
enum UserKey {
    Table,
    Id,
    UserId,
    CreatedAt,
    Key,
    Metadata,
}

#[derive(DeriveIden)]
enum Profile {
    Table,
    Id,
    OwningUserId,
    Name,
    Discriminator,
    DisplayName,
    Picture,
    Banner,
    Metadata,
}

#[derive(DeriveIden)]
enum Guild {
    Table,
    Id,
    CreatedAt,
    Name,
    OwnerId,
}

#[derive(DeriveIden, Sequence)]
enum ChannelType {
    Table,
    Dummy,
    Text,
    Voice,
    Feed,
    Announcement,
    Forum,
}

#[derive(DeriveIden)]
enum Channel {
    Table,
    Id,
    GuildId,
    ParentId,
    CreatedAt,
    Name,
    Type,
    Position,
    Topic,
    Metadata,
}

#[derive(DeriveIden, Sequence)]
enum Flags {
    Table,
    //content is not safe for work (NSFW)
    //  this advises users to discretion or avoid the content
    Nsfw,
    //content contains spoilers
    //  this allows users to avoid being spoiled
    Spoilers,
    //content that is generated by AI
    //  this allows users to filter out content that they don't want to see
    AiGenerated,
    //ragable posts like Politics, Religion, etc.
    //  this is intended to allow users to filter out content that is likely to make them angry
    Rageable,
    //content that is fake news
    // example moderator can mark a post as fake news
    FakeNews,
    //content is managed by a bot or script or something similar
    // example could be a bot that posts weather updates
    Automated,
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    ChannelId,
    AuthorId,
    CreatedAt,
    UpdatedAt,
    Title,
    Body,
    Tags,
    Flags,
    Metadata,
}

#[derive(DeriveIden)]
enum PostAttachment {
    Table,
    PostId,
    AssetId,
    Position,
}
