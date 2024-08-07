//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "asset_processing_task"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub task_id: i64,
    pub asset_id: Uuid,
    pub created_at: DateTime,
    pub started_at: Option<DateTime>,
    pub finished_at: Option<DateTime>,
    pub progress: f64,
    pub configuration: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    TaskId,
    AssetId,
    CreatedAt,
    StartedAt,
    FinishedAt,
    Progress,
    Configuration,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    TaskId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::TaskId => ColumnType::BigInteger.def(),
            Self::AssetId => ColumnType::Uuid.def(),
            Self::CreatedAt => ColumnType::DateTime.def(),
            Self::StartedAt => ColumnType::DateTime.def().null(),
            Self::FinishedAt => ColumnType::DateTime.def().null(),
            Self::Progress => ColumnType::Double.def(),
            Self::Configuration => ColumnType::JsonBinary.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
