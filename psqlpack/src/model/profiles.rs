//! Profiles are configurations that affect how an operation is applied.
//!
//! For instance, a `PublishProfile` might determine how unknown entities in the
//! target are handled when performing a `publish` operation.

use std::default::Default;
use std::path::Path;
use std::fs::File;

use serde_json;

use errors::{PsqlpackResult, PsqlpackResultExt};
use errors::PsqlpackErrorKind::*;
use super::PackageParameter;

#[derive(Deserialize, Serialize)]
pub struct PublishProfile {
    pub version: String,
    #[serde(rename = "generationOptions")] pub generation_options: GenerationOptions,
    #[serde(default, rename = "packageParameters")] pub package_parameters: Vec<PackageParameter>,
}

#[derive(Deserialize, Serialize)]
pub enum Toggle {
    Allow,
    Ignore,
    Error,
}

#[derive(Deserialize, Serialize)]
pub struct GenerationOptions {
    /// If set to true, the database will always be recereated
    #[serde(rename = "alwaysRecreateDatabase")] pub always_recreate_database: bool,

    /// Enum values are typically unsafe to delete. If set to Allow, psqlpack will attempt to delete.
    /// Default: Error
    #[serde(rename = "dropEnumValues")] pub drop_enum_values: Toggle,
    /// Tables may have data in them which may not be intended to be deleted. If set to Allow, psqlpack will drop the table.
    /// Default: Error
    #[serde(rename = "dropTables")] pub drop_tables: Toggle,
    /// Columns may have data in them which may not be intended to be deleted. If set to Allow, psqlpack will drop the column.
    /// Default: Error
    #[serde(rename = "dropColumns")] pub drop_columns: Toggle,
    /// Primary Keys define how a table is looked up on disk. If set to Allow, psqlpack will drop the primary key.
    /// Default: Error
    #[serde(rename = "dropPrimaryKeyConstraints")] pub drop_primary_key_constraints: Toggle,
    /// Foreign Keys define a constraint to another table. If set to Allow, psqlpack will drop the foreign key.
    /// Default: Allow
    #[serde(rename = "dropForeignKeyConstraints")] pub drop_foreign_key_constraints: Toggle,
    /// Functions may not be intended to be deleted. If set to Allow, psqlpack will drop the function.
    /// Default: Error
    #[serde(rename = "dropFunctions")] pub drop_functions: Toggle,
    /// Indexes may not be intended to be deleted. If set to Allow, psqlpack will drop the index.
    /// Default: Allow
    #[serde(rename = "dropIndexes")] pub drop_indexes: Toggle,

    /// Forces index changes to be made concurrently to avoid locking on table writes.
    /// Default: true
    #[serde(rename = "forceConcurrentIndexes")] pub force_concurrent_indexes: bool,
}

impl Default for PublishProfile {
    fn default() -> Self {
        PublishProfile {
            version: "1.0".to_owned(),
            package_parameters: Vec::new(),
            generation_options: GenerationOptions {
                always_recreate_database: false,

                drop_enum_values: Toggle::Error,
                drop_tables: Toggle::Error,
                drop_columns: Toggle::Error,
                drop_primary_key_constraints: Toggle::Error,
                drop_foreign_key_constraints: Toggle::Allow,
                drop_functions: Toggle::Error,
                drop_indexes: Toggle::Allow,

                force_concurrent_indexes: true,
            },
        }
    }
}

impl PublishProfile {
    pub fn from_path(profile_path: &Path) -> PsqlpackResult<PublishProfile> {
        File::open(profile_path)
            .chain_err(|| PublishProfileReadError(profile_path.to_path_buf()))
            .and_then(|file| {
                serde_json::from_reader(file).chain_err(|| PublishProfileParseError(profile_path.to_path_buf()))
            })
    }
}
