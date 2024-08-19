use sea_orm_migration::prelude::*;

use crate::extension::postgres::{TypeCreateStatement, TypeDropStatement};
use crate::sea_orm::{DatabaseBackend, EnumIter, Iterable};

#[derive(DeriveMigrationName)]
pub struct Migration;

//TODO citus support
//TODO mysql/mariadb support?
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Configuration::Table)
                    .col(
                        ColumnDef::new(Configuration::Name)
                            .string()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Configuration::Value)
                            .json_binary()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

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
                    .col(
                        ColumnDef::new(Account::Email)
                            .unique_key()
                            .string_len(320)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Account::Password)
                            .text()
                            .null()
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AccountKey::Table)
                    .col(ColumnDef::new(AccountKey::Id).uuid().primary_key())
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
                    .col(
                        ColumnDef::new(Asset::Public)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AssetProcessingTask::Table)
                    .col(
                        ColumnDef::new(AssetProcessingTask::TaskId)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AssetProcessingTask::AssetId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssetProcessingTask::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssetProcessingTask::StartedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(AssetProcessingTask::FinishedAt)
                            .timestamp()
                            .null(),
                    )
                    //progress is a double between 0.0 and 1.0
                    .col(
                        ColumnDef::new(AssetProcessingTask::Progress)
                            .double()
                            .not_null()
                            .default(0.0),
                    )
                    //override default task settings with specific task configuration
                    .col(
                        ColumnDef::new(AssetProcessingTask::Configuration)
                            .json_binary()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .col(ColumnDef::new(Profile::Id).big_integer().primary_key())
                    .col(
                        ColumnDef::new(Profile::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Profile::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Profile::Name).string().not_null())
                    .col(
                        ColumnDef::new(Profile::Discriminator)
                            .small_unsigned()
                            .null(),
                    )
                    .col(ColumnDef::new(Profile::DisplayName).string().null())
                    .col(ColumnDef::new(Profile::Avatar).big_integer().null())
                    .col(ColumnDef::new(Profile::Banner).big_integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(Profile::FkProfileAvatarAssetId.to_string())
                            .from(Profile::Table, Profile::Avatar)
                            .to(Asset::Table, Asset::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(Profile::FkProfileBannerAssetId.to_string())
                            .from(Profile::Table, Profile::Banner)
                            .to(Asset::Table, Asset::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .table(Profile::Table)
                            .col(Profile::Name)
                            .col(Profile::Discriminator)
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
                            .name(Asset::FkMediaOriginProfileId.to_string())
                            .from_tbl(Asset::Table)
                            .from_col(Asset::Origin)
                            .to_tbl(Profile::Table)
                            .to_col(Profile::Id)
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
                            .to(Profile::Table, Profile::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                TypeCreateStatement::new()
                    .values(ChannelTypeEnum::iter().skip(1).collect::<Vec<_>>())
                    .as_enum(ChannelTypeEnum::Entity)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Channel::Table)
                    .col(ColumnDef::new(Channel::Id).big_integer().primary_key())
                    .col(
                        ColumnDef::new(Channel::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Channel::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Channel::Data)
                            .json_binary()
                            .not_null()
                            .default("{}".to_owned()),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(GuildChannel::Table)
                    .col(ColumnDef::new(GuildChannel::Id).big_integer().primary_key())
                    .col(
                        ColumnDef::new(GuildChannel::Type)
                            .custom(ChannelTypeEnum::Entity)
                            .not_null(),
                    )
                    .col(ColumnDef::new(GuildChannel::Name).string().not_null())
                    .col(
                        ColumnDef::new(GuildChannel::GuildId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(GuildChannel::ParentId).big_integer().null())
                    .col(
                        ColumnDef::new(GuildChannel::Order)
                            .small_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(GuildChannel::FkGuildChannelChannelId.to_string())
                            .from(GuildChannel::Table, GuildChannel::Id)
                            .to(Channel::Table, Channel::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(GuildChannel::FkGuildChannelParentId.to_string())
                            .from(GuildChannel::Table, GuildChannel::ParentId)
                            .to(GuildChannel::Table, GuildChannel::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(GuildChannel::FkGuildChannelGuildId.to_string())
                            .from(GuildChannel::Table, GuildChannel::GuildId)
                            .to(Guild::Table, Guild::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .table(GuildChannel::Table)
                            .col(GuildChannel::GuildId)
                            .col(GuildChannel::ParentId)
                            .col(GuildChannel::Name)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Publication::Table)
                    .col(ColumnDef::new(Publication::Id).big_integer().primary_key())
                    .col(
                        ColumnDef::new(Publication::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Publication::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Publication::AuthorId)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(Publication::FkPublicationAuthorId.to_string())
                            .from(Publication::Table, Publication::AuthorId)
                            .to(Profile::Table, Profile::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Attachment::Table)
                    .col(
                        ColumnDef::new(Attachment::PublicationId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Attachment::AssetId).big_integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(Attachment::PublicationId)
                            .col(Attachment::AssetId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(Attachment::FkAttatchmentPublicationId.to_string())
                            .from(Attachment::Table, Attachment::PublicationId)
                            .to(Publication::Table, Publication::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(Attachment::FkAttatchmentAssetId.to_string())
                            .from(Attachment::Table, Attachment::AssetId)
                            .to(Asset::Table, Asset::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Attachment::Order)
                            .small_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                TypeCreateStatement::new()
                    .values(ContentFlagEnum::iter().skip(1).collect::<Vec<_>>())
                    .as_enum(ContentFlagEnum::Entity)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ContentFlag::Table)
                    .col(
                        ColumnDef::new(ContentFlag::PublicationId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ContentFlag::Flag)
                            .custom(ContentFlagEnum::Entity)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ContentFlag::Details)
                            .json_binary()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(ContentFlag::PublicationId)
                            .col(ContentFlag::Flag),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(ContentFlag::FkPostFlagPostId.to_string())
                            .from(ContentFlag::Table, ContentFlag::PublicationId)
                            .to(Publication::Table, Publication::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .col(ColumnDef::new(Post::Id).big_integer().primary_key())
                    .col(ColumnDef::new(Post::Draft).boolean().not_null())
                    .col(ColumnDef::new(Post::Title).string().null())
                    .col(ColumnDef::new(Post::Content).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(Post::FkPostPublicationId.to_string())
                            .from(Post::Table, Post::Id)
                            .to(Publication::Table, Publication::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ChannelMessage::Table)
                    .col(
                        ColumnDef::new(ChannelMessage::Id)
                            .big_integer()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ChannelMessage::ChannelId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ChannelMessage::ReplyTo).big_integer().null())
                    .col(
                        ColumnDef::new(ChannelMessage::Overwrites)
                            .big_integer()
                            .null(),
                    )
                    .col(ColumnDef::new(ChannelMessage::Content).text().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(ChannelMessage::FkMessageChannelId.to_string())
                            .from(ChannelMessage::Table, ChannelMessage::ChannelId)
                            .to(Channel::Table, Channel::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(ChannelMessage::FkMessagePublicationId.to_string())
                            .from(ChannelMessage::Table, ChannelMessage::Id)
                            .to(Publication::Table, Publication::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(ChannelMessage::FkMessageOverwrites.to_string())
                            .from(ChannelMessage::Table, ChannelMessage::Overwrites)
                            .to(ChannelMessage::Table, ChannelMessage::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop Publications

        manager
            .drop_table(
                Table::drop()
                    .table(ChannelMessage::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Post::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ContentFlag::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_type(
                TypeDropStatement::new()
                    .if_exists()
                    .name(ContentFlagEnum::Entity)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Attachment::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(Publication::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        // Drop channels and guilds
        manager
            .drop_table(
                Table::drop()
                    .table(GuildChannel::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_type(
                TypeDropStatement::new()
                    .if_exists()
                    .name(ChannelTypeEnum::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Channel::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Guild::Table).if_exists().to_owned())
            .await?;

        // Drop profiles and accounts
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Asset::Table)
                    .name(Asset::FkMediaOriginProfileId.to_string())
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Profile::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(AccountKey::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Account::Table).if_exists().to_owned())
            .await?;

        // Drop everything else
        manager
            .drop_table(
                Table::drop()
                    .table(AssetProcessingTask::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Asset::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(Configuration::Table)
                    .if_exists()
                    .to_owned(),
            )
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
    Password,
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
    FkMediaOriginProfileId,
}

#[derive(DeriveIden)]
enum AssetProcessingTask {
    Table,
    TaskId,
    AssetId,
    CreatedAt,
    StartedAt,
    FinishedAt,
    Progress,
    Configuration,
    FkAssetProcessingTaskAssetId,
}

#[derive(DeriveIden)]
enum Profile {
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
    FkProfileAvatarAssetId,
    FkProfileBannerAssetId,
}

#[derive(DeriveIden)]
enum Guild {
    #[sea_orm(iden = "guild")]
    Table,
    Id,
    CreatedAt,
    OwnerId,
    Name,
    FkGuildOwnerId,
}

#[derive(DeriveIden, EnumIter)]
enum ChannelTypeEnum {
    #[sea_orm(iden = "channel_type")]
    Entity,
    Dummy,
    Text,
    Voice,
    Feed,
    Forum,
    Stage,
}

#[derive(DeriveIden)]
enum Channel {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Data,
}
#[derive(DeriveIden)]
enum GuildChannel {
    Table,
    Id,
    Type,
    Name,
    GuildId,
    ParentId,
    Order,
    FkGuildChannelChannelId,
    FkGuildChannelParentId,
    FkGuildChannelGuildId,
}

#[derive(DeriveIden)]
enum Publication {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    AuthorId,
    FkPublicationAuthorId,
}

#[derive(DeriveIden)]
enum Attachment {
    Table,
    Order,
    PublicationId,
    AssetId,
    FkAttatchmentPublicationId,
    FkAttatchmentAssetId,
}

#[derive(DeriveIden, EnumIter)]
enum ContentFlagEnum {
    #[sea_orm(iden = "content_flag_type")]
    Entity,
    Spam,
    Nsfw,
    //like nsfw but more intense
    NsfwX,
    //like nsfwx but even more intense
    NsfwXX,
    //like nsfwxx but the most intense
    NsfwXXX,
    FakeNews,
    AiGenerated,
    //basicaly bot generated content
    Automated,
}

#[derive(DeriveIden)]
enum ContentFlag {
    Table,
    PublicationId,
    Flag,
    Details,
    FkPostFlagPostId,
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Draft,
    Id,
    Title,
    Content,
    FkPostPublicationId,
}

#[derive(DeriveIden)]
enum ChannelMessage {
    Table,
    Id,
    ChannelId,
    ReplyTo,
    Overwrites,
    Content,
    FkMessageReplyTo,
    FkMessageOverwrites,
    FkMessageChannelId,
    FkMessagePublicationId,
}
