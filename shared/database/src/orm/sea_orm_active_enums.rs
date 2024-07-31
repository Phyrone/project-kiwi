//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "channel_type")]
pub enum ChannelType {
    #[sea_orm(string_value = "dummy")]
    Dummy,
    #[sea_orm(string_value = "feed")]
    Feed,
    #[sea_orm(string_value = "forum")]
    Forum,
    #[sea_orm(string_value = "stage")]
    Stage,
    #[sea_orm(string_value = "text")]
    Text,
    #[sea_orm(string_value = "voice")]
    Voice,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "content_flag_type")]
pub enum ContentFlagType {
    #[sea_orm(string_value = "ai_generated")]
    AiGenerated,
    #[sea_orm(string_value = "fake_news")]
    FakeNews,
    #[sea_orm(string_value = "nsfw")]
    Nsfw,
    #[sea_orm(string_value = "nsfw_x")]
    NsfwX,
    #[sea_orm(string_value = "nsfw_xx")]
    NsfwXx,
    #[sea_orm(string_value = "nsfw_xxx")]
    NsfwXxx,
    #[sea_orm(string_value = "spam")]
    Spam,
}
