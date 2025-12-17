use std::collections::{HashMap, HashSet};

use email_address::EmailAddress;
use semver::{Version, VersionReq};
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub struct ShardConfig {
    pub shard: ShardMetadata,
    pub authors: ShardAuthorList,
    #[serde(default)]
    pub relations: Vec<ShardRelation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShardEdition {
    V0,
}

#[derive(Deserialize)]
pub struct ShardMetadata {
    pub slug: String,
    pub namespace: String,
    pub target: String,
    pub version: Version,
    pub edition: ShardEdition,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub repository: Option<Url>,
    #[serde(default)]
    pub issues: Option<Url>,
    #[serde(default)]
    pub website: Option<Url>,
    #[serde(default)]
    pub keywords: HashSet<String>,
}

#[derive(Deserialize, Default)]
pub struct ShardAuthorList(pub HashMap<String, ShardAuthor>);

#[derive(Deserialize)]
pub struct ShardAuthor {
    #[serde(default)]
    pub discord: Option<String>,
    #[serde(default)]
    pub email: Option<EmailAddress>,
    #[serde(default)]
    pub url: Option<Url>,
}

#[derive(Deserialize)]
pub struct ShardRelation {
    pub version: VersionReq,
    pub from: String,
    #[serde(default)]
    pub kind: ShardRelationKind,
    #[serde(default)]
    pub order: Option<ShardRelationOrder>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ShardRelationKind {
    #[default]
    Required,
    Recommended,
    Optional,
    Dev,
    Incompatible,
    Replaces,
    ConflictsPartially,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ShardRelationOrder {
    #[default]
    After,
    Before,
}
