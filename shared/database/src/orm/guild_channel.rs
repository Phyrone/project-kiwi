//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "guild_channel"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub order: i16,
    pub created_at: DateTimeWithTimeZone,
    pub guild_id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub data: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Order,
    CreatedAt,
    GuildId,
    ParentId,
    Name,
    Data,
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
    Guild,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::Order => ColumnType::SmallInteger.def(),
            Self::CreatedAt => ColumnType::TimestampWithTimeZone.def(),
            Self::GuildId => ColumnType::BigInteger.def(),
            Self::ParentId => ColumnType::BigInteger.def().null(),
            Self::Name => ColumnType::String(None).def(),
            Self::Data => ColumnType::JsonBinary.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Guild => Entity::belongs_to(super::guild::Entity)
                .from(Column::GuildId)
                .to(super::guild::Column::Id)
                .into(),
        }
    }
}

impl Related<super::guild::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Guild.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}