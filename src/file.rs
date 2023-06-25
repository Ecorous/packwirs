use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Pack {
    pub name: String,
    pub author: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub pack_format: String,
    pub index: PackFileIndex,
    pub versions: PackFileVersion,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct PackFileIndex {
    pub file: String,
    pub hash_format: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct PackFileVersion {
    pub quilt: Option<String>,
    pub fabric: Option<String>,
    pub forge: Option<String>,
    pub liteloader: Option<String>,
    pub minecraft: String,
}
