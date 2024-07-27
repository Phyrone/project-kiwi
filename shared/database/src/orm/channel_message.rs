//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "channel_message"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub channel_id: i64,
    pub reply_to: Option<i64>,
    pub overwrites: Option<i64>,
    pub content: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    ChannelId,
    ReplyTo,
    Overwrites,
    Content,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Channel,
    SelfRef,
    Publication,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::ChannelId => ColumnType::BigInteger.def(),
            Self::ReplyTo => ColumnType::BigInteger.def().null(),
            Self::Overwrites => ColumnType::BigInteger.def().null(),
            Self::Content => ColumnType::Text.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Channel => Entity::belongs_to(super::channel::Entity)
                .from(Column::ChannelId)
                .to(super::channel::Column::Id)
                .into(),
            Self::SelfRef => Entity::belongs_to(Entity)
                .from(Column::Overwrites)
                .to(Column::Id)
                .into(),
            Self::Publication => Entity::belongs_to(super::publication::Entity)
                .from(Column::Id)
                .to(super::publication::Column::Id)
                .into(),
        }
    }
}

impl Related<super::channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Channel.def()
    }
}

impl Related<super::publication::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Publication.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
