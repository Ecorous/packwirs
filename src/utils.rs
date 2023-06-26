use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MCVersions {
    //latest: LatestMCVersion,
    versions: Vec<MCVersion>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MCVersion {
    id: String,
    #[serde(rename = "type")]
    type_: MCVersionType,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum MCVersionType {
    Release,
    Snapshot,
    OldBeta,
    OldAlpha,
}

#[derive(Serialize, Deserialize, Debug)]
struct LatestMCVersion {
    latest: String,
    snapshot: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct QLoaderVersion {
    separator: String,
    build: i64,
    maven: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FLoaderVersion {
    separator: String,
    build: i64,
    maven: String,
    version: String,
    stable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ForgeVersionR {
     metadata: Metadata
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "metadata")]
struct Metadata {
    versioning: Versioning,
}
#[derive(Serialize, Deserialize, Debug)]
struct Versioning {
    versions: ForgeVersion,
}

#[derive(Serialize, Deserialize, Debug)]
struct ForgeVersion {
    version: Vec<String>
}

pub(crate) fn get_quilt_versions(client: Client) -> Vec<String> {
    let versions: Vec<QLoaderVersion> = client
        .get("https://meta.quiltmc.org/v3/versions/loader")
        .send()
        .unwrap()
        .json()
        .unwrap();
    let mut vec = vec![];
    for version in versions {
        vec.append(&mut vec![version.version])
    }
    return vec;
}
pub(crate) fn get_fabric_versions(client: Client) -> Vec<String> {
    let versions: Vec<FLoaderVersion> = client
        .get("https://meta.fabricmc.net/v2/versions/loader")
        .send()
        .unwrap()
        .json()
        .unwrap();
    let mut vec = vec![];
    for version in versions {
        vec.append(&mut vec![version.version])
    }
    return vec;
}
pub(crate) fn get_forge_versions(client: Client) -> Vec<String> {
    let binding = xmltojson::to_json(
        client
            .get("https://maven.minecraftforge.net/net/minecraftforge/forge/maven-metadata.xml")
            .send()
            .unwrap()
            .text()
            .unwrap()
            .as_str(),
    );
    let nbinding = binding.unwrap();
    let abinding = nbinding.to_string();
    let next_out_json_response  = abinding.as_str();
    let forge: ForgeVersionR = serde_json::from_str(next_out_json_response)
    .unwrap();
    let mut vec = vec![];
    for version in forge.metadata.versioning.versions.version {
        vec.append(&mut vec![version.split("-").last().unwrap().to_owned()])
    }
    return vec; // TODO add proper forge version getting
}
pub(crate) fn get_liteloader_versions(client: Client) -> Vec<String> {
    return vec![String::from("1.12.2")]; // TODO add proper liteloader version getting
}
pub(crate) fn get_minecraft_versions(client: Client) -> Vec<String> {
    let versions: MCVersions = client
        .get("https://piston-meta.mojang.com/mc/game/version_manifest.json")
        .send()
        .unwrap()
        .json()
        .unwrap();
    let mut vec = vec![];
    for version in versions.versions {
        vec.append(&mut vec![version.id])
    }
    return vec;
}

pub(crate) fn to_modloader(string: String) -> crate::Modloader {
    match string.to_ascii_lowercase().as_str() {
        "quilt" => crate::Modloader::Quilt,
        "fabric" => crate::Modloader::Fabric,
        "forge" => crate::Modloader::Forge,
        "liteloader" => crate::Modloader::Liteloader,
        _ => crate::Modloader::Vanilla,
    }
}
