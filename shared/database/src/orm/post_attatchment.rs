//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "post_attatchment"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub post_id: i64,
    pub asset_id: Uuid,
    pub order: i16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    PostId,
    AssetId,
    Order,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    PostId,
    AssetId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (i64, Uuid);
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Asset,
    Post,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::PostId => ColumnType::BigInteger.def(),
            Self::AssetId => ColumnType::Uuid.def(),
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
            Self::Post => Entity::belongs_to(super::post::Entity)
                .from(Column::PostId)
                .to(super::post::Column::Id)
                .into(),
        }
    }
}

impl Related<super::asset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Asset.def()
    }
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
