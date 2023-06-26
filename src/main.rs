#![allow(clippy::large_enum_variant)]
use crate::file::{Pack, PackFileIndex, PackFileVersion};
use clap::{Parser, Subcommand};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::{Path, PathBuf};
mod file;
mod utils;

enum Modloader {
    Quilt,
    Fabric,
    Forge,
    Liteloader,
    Vanilla,
}

impl std::fmt::Display for Modloader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Modloader::Quilt => write!(f, "quilt"),
            Modloader::Fabric => write!(f, "fabric"),
            Modloader::Forge => write!(f, "forge"),
            Modloader::Liteloader => write!(f, "liteloader"),
            Modloader::Vanilla => write!(f, "vanilla"),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(about, version)]
#[clap(propagate_version = true)]
struct Packwirs {
    #[command(subcommand)]
    subcommand: Command,

    #[arg(
        long,
        help = "path to pack directory. defaults to current directory (.)"
    )]
    pack_dir: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        author: Option<String>,
        #[arg(long)]
        pack_version: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        mc_version: Option<String>,
        #[arg(long)]
        modloader: Option<String>,
        #[arg(long)]
        quilt_version: Option<String>,
        #[arg(long)]
        fabric_version: Option<String>,
        #[arg(long)]
        forge_version: Option<String>,
        #[arg(long)]
        liteloader_version: Option<String>,
    },
    #[command(aliases = vec!["cf", "curse"])]
    Curseforge {
        #[command(subcommand)]
        subcommand: CFCommand,
    },
    Read,
}

#[derive(Subcommand, Debug)]
enum CFCommand {
    #[command(aliases = vec!["install", "get"])]
    Add {
        #[arg()]
        project: String,
    },
    Detect {},
    Export {},
    Import {},
    #[command(alias = "doc")]
    Open {},
}

#[derive(Subcommand, Debug)]
enum MRCommand {
    #[command(aliases = vec!["install", "get"])]
    Add {
        #[arg()]
        project: String,
    },
    Export {
        #[arg(long, short = 'o')]
        output: String,
        #[arg(long)]
        unrestrict_domains: bool,
    },
}

fn pack_toml_exists(dir: &Path) -> bool {
    dir.join("pack.toml").exists()
}

fn print_debug_info(pack: Pack) {
    println!("name: {:?}", pack.name);
    println!("author: {:?}", pack.author);
    println!("description: {:?}", pack.description);
    println!("version: {:?}", pack.version);
    println!("pack-format: {:?}", pack.pack_format);
    println!("index.file: {:?}", pack.index.file);
    println!("index.hash-format: {:?}", pack.index.hash_format);
    println!("index.hash: {:?}", pack.index.hash);
    println!("versions.minecraft: {:?}", pack.versions.minecraft);
    println!("versions.quilt: {:?}", pack.versions.quilt);
    println!("versions.fabric: {:?}", pack.versions.fabric);
    println!("versions.forge: {:?}", pack.versions.forge);
    println!("versions.liteloader: {:?}", pack.versions.liteloader)
}

fn main() {
    let pack_format = "packwiz:1.1.0";
    let cli = Packwirs::parse();
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        ))
        .build().unwrap();
    match cli.subcommand {
        Command::Curseforge { subcommand } => match subcommand {
            CFCommand::Add { project } => {
                println!("Reach cf add, do stuff late™. project: {project}")
            }
            CFCommand::Detect {} => {
                todo!()
            }
            CFCommand::Export {} => {
                todo!()
            }
            CFCommand::Import {} => {
                todo!()
            }
            CFCommand::Open {} => {
                todo!()
            }
        },
        Command::Read => {
            println!("starting read");
            let mut finalpack = PathBuf::from(".");
            if let Some(newpack) = &cli.pack_dir {
                finalpack = PathBuf::from(newpack);
            }
            if !pack_toml_exists(&finalpack) {
                panic!("YOU DID A BAD. THERE IS NO PACKTOML HERE MOTHERFUCKER");
            }
            let pack_toml = finalpack.join("pack.toml");
            println!("reading from {}", pack_toml.display());
            let pack: Pack = toml::from_str(
                &read_to_string(pack_toml).expect("not proper error handling because :uwu:"),
            )
            .unwrap();
            print_debug_info(pack)
        }
        Command::Init {
            name,
            author,
            pack_version: version,
            description,
            mc_version,
            modloader,
            quilt_version,
            fabric_version,
            forge_version,
            liteloader_version,
        } => {
            let mut finalpack = PathBuf::from(".");
            if let Some(newpack) = &cli.pack_dir {
                finalpack = PathBuf::from(newpack);
            }
            if pack_toml_exists(&finalpack) {
                panic!("A pack.toml already exists here. Not continuing with `init`")
            }
            let pack_toml = finalpack.join("pack.toml");
            let final_name: String;
            let final_author: Option<String>;
            let final_version: Option<String>;
            let final_description: Option<String>;
            let final_mc_version: String;
            let final_modloader: Modloader;
            let mut final_quilt_version: Option<String>;
            let mut final_fabric_version: Option<String>;
            let mut final_forge_version: Option<String>;
            let mut final_liteloader_version: Option<String>;
            if let Some(ref name_some) = name {
                final_name = name_some.to_string()
            } else {
                final_name = inquire::Text::new("Pack Name").prompt().unwrap();
            }
            if let Some(ref author_some) = author {
                final_author = Some(author_some.to_string())
            } else {
                final_author = inquire::Text::new("Author").prompt_skippable().unwrap();
            }
            if let Some(ref version_some) = version {
                final_version = Some(version_some.to_string())
            } else {
                final_version = inquire::Text::new("Pack Version")
                    .prompt_skippable()
                    .unwrap();
            }
            if let Some(ref description_some) = description {
                final_description = Some(description_some.to_string())
            } else {
                final_description = inquire::Text::new("Pack Description")
                    .prompt_skippable()
                    .unwrap();
            }
            if let Some(ref mc_version_some) = mc_version {
                final_mc_version = mc_version_some.to_string()
            } else {
                final_mc_version = inquire::Select::new("Minecraft Version", utils::get_minecraft_versions(client.clone())).prompt().unwrap();
            }
            if let Some(ref modloader_some) = modloader {
                final_modloader = utils::to_modloader(modloader_some.to_string())
            } else {
                final_modloader = inquire::Select::new(
                    "Modloader",
                    vec![
                        Modloader::Quilt,
                        Modloader::Fabric,
                        Modloader::Forge,
                        Modloader::Liteloader,
                        Modloader::Vanilla,
                    ],
                )
                .prompt()
                .unwrap();
            }
            final_quilt_version = None;
            final_fabric_version = None;
            final_forge_version = None;
            final_liteloader_version = None;
            match final_modloader {
                Modloader::Quilt => {
                    if quilt_version.is_some() {
                        final_quilt_version = quilt_version;
                    } else {
                        final_quilt_version = Some(
                            inquire::Select::new(
                                "Quilt Version",
                                utils::get_quilt_versions(client.clone()),
                            )
                            .prompt()
                            .unwrap(),
                        );
                    }
                }
                Modloader::Fabric => {
                    if fabric_version.is_some() {
                        final_fabric_version = fabric_version;
                    } else {
                        final_fabric_version = Some(
                            inquire::Select::new(
                                "Fabric Version",
                                utils::get_fabric_versions(client.clone()),
                            )
                            .prompt()
                            .unwrap(),
                        );
                    }
                }
                Modloader::Forge => {
                    if forge_version.is_some() {
                        final_forge_version = forge_version;
                    } else {
                        println!("Starting forge read - this may take a while");
                        final_forge_version = Some(
                            inquire::Select::new(
                                "Forge Version",
                                utils::get_forge_versions(client.clone()),
                            )
                            .prompt()
                            .unwrap(),
                        );
                    }
                }
                Modloader::Liteloader => {
                    if liteloader_version.is_some() {
                        final_liteloader_version = liteloader_version;
                    } else {
                        final_liteloader_version = Some(
                            inquire::Select::new(
                                "Liteloader Version",
                                utils::get_liteloader_versions(client.clone()),
                            )
                            .prompt()
                            .unwrap(),
                        );
                    }
                }
                Modloader::Vanilla => {}
            }
            let pack = Pack {
                name: final_name,
                author: final_author,
                version: final_version,
                description: final_description,
                pack_format: pack_format.to_string(),
                index: PackFileIndex {
                    file: String::from("index.toml"),
                    hash_format: String::from("sha256"),
                    hash: String::from("I'm not a valid hash :P"),
                },
                versions: PackFileVersion {
                    minecraft: final_mc_version,
                    quilt: final_quilt_version,
                    fabric: final_fabric_version,
                    forge: final_forge_version,
                    liteloader: final_liteloader_version,
                },
            };
            let toml = toml::to_string(&pack).unwrap();
            let mut file = File::create(pack_toml).unwrap();
            let _ = file.write_all(toml.as_bytes());
        }
    }
    println!("Hello, world!");
}
