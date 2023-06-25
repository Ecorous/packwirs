
pub(crate) fn get_quilt_versions() -> Vec<String> {
    return vec![String::from("0.19.2-beta.2")]
}
pub(crate) fn get_fabric_versions() -> Vec<String> {
    return vec![String::from("0.14.21")]
}
pub(crate) fn get_forge_versions() -> Vec<String> {
    return vec![String::from("47.0.19")]
}
pub(crate) fn get_liteloader_versions() -> Vec<String> {
    return vec![String::from("1.12.2")]
}

pub(crate) fn to_modloader(string: String) -> crate::Modloader {
    match string.to_ascii_lowercase().as_str() {
        "quilt" => crate::Modloader::Quilt,
        "fabric" => crate::Modloader::Fabric,
        "forge" => crate::Modloader::Forge,
        "liteloader" => crate::Modloader::Liteloader,
        _ => crate::Modloader::Vanilla
    }
}