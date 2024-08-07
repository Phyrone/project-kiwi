//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "attachment"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub publication_id: i64,
    pub asset_id: i64,
    pub order: i16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    PublicationId,
    AssetId,
    Order,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    PublicationId,
    AssetId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (i64, i64);
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Asset,
    Publication,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::PublicationId => ColumnType::BigInteger.def(),
            Self::AssetId => ColumnType::BigInteger.def(),
            Self::Order => ColumnType::SmallInteger.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Asset => Entity::belongs_to(super::asset::Entity)
                .from(Column::AssetId)
                .to(super::asset::Column::Id)
                .into(),
            Self::Publication => Entity::belongs_to(super::publication::Entity)
                .from(Column::PublicationId)
                .to(super::publication::Column::Id)
                .into(),
        }
    }
}

impl Related<super::asset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Asset.def()
    }
}

impl Related<super::publication::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Publication.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
