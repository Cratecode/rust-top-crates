#![deny(rust_2018_idioms)]

use rust_playground_top_crates::*;
use serde::Serialize;
use std::{
    collections::BTreeMap,
    fs::{self, File},
    path::{Path, PathBuf},
};

/// A Cargo.toml file.
#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct TomlManifest {
    package: TomlPackage,
    dependencies: BTreeMap<String, DependencySpec>,
}

/// Header of Cargo.toml file.
#[derive(Serialize)]
struct TomlPackage {
    name: String,
    version: String,
    edition: String,
}

fn main() {
    let d = fs::read_to_string("crate-modifications.toml")
        .expect("unable to read crate modifications file");

    let modifications: Modifications =
        toml::from_str(&d).expect("unable to parse crate modifications file");

    let (dependencies, infos) = rust_playground_top_crates::generate_info(&modifications);

    // Construct playground's Cargo.toml.
    let manifest = TomlManifest {
        package: TomlPackage {
            name: "project".to_owned(),
            version: "0.1.0".to_owned(),
            edition: "2021".to_owned(),
        },
        dependencies: dependencies.clone(),
    };

    // Write manifest file.
    let base_directory: PathBuf = std::env::args_os()
        .nth(1)
        .unwrap_or_else(|| "../output".into())
        .into();

    let cargo_toml = base_directory.join("Cargo.toml");
    write_manifest(manifest, &cargo_toml);
    println!("wrote {}", cargo_toml.display());

    let path = base_directory.join("crate-information.json");
    let mut f = File::create(&path)
        .unwrap_or_else(|e| panic!("Unable to create {}: {}", path.display(), e));
    serde_json::to_writer_pretty(&mut f, &infos)
        .unwrap_or_else(|e| panic!("Unable to write {}: {}", path.display(), e));
    println!("Wrote {}", path.display());
}

fn write_manifest(manifest: TomlManifest, path: impl AsRef<Path>) {
    let content = toml::to_string(&manifest).expect("Couldn't serialize TOML");
    fs::write(path, content).expect("Couldn't write Cargo.toml");
}
