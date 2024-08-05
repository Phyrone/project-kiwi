//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "account"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub session_secret: Vec<u8>,
    pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    CreatedAt,
    UpdatedAt,
    SessionSecret,
    Email,
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
    AccountKey,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::CreatedAt => ColumnType::TimestampWithTimeZone.def(),
            Self::UpdatedAt => ColumnType::TimestampWithTimeZone.def(),
            Self::SessionSecret => ColumnType::VarBinary(StringLen::None).def(),
            Self::Email => ColumnType::String(StringLen::N(320u32)).def().unique(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AccountKey => Entity::has_many(super::account_key::Entity).into(),
        }
    }
}

impl Related<super::account_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccountKey.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
