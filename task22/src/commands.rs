use fs_extra::dir::{CopyOptions, copy, remove as remove_package};
use semver::{Version, VersionReq};
use std::{
    error::Error,
    fs::{self, File},
    path::PathBuf,
};

use crate::manifest::{
    Dependency::{self},
    create_cargo, read_cargo,
};

// create myproject
pub fn init() -> Result<(), Box<dyn Error>> {
    let create_dir = "my_project/src";
    fs::create_dir_all(create_dir)?;
    let create_files = vec![
        "my_project/src/main.rs".to_string(),
        "my_project/Cargo.toml".to_string(),
    ];
    create_files
        .iter()
        .try_for_each(|f| -> Result<(), Box<dyn Error>> {
            File::create(f)?;
            let path = f.to_string();
            if path.contains("Cargo.toml") {
                create_cargo(path)?;
            } else if path.contains("main.rs") {
                let main_content = "fn main() {\n\tprintln!(\"Hello, world!\");\n}";
                fs::write(path, main_content)?;
            }
            Ok(())
        })?;

    Ok(())
}

pub fn read() -> Result<(), Box<dyn Error>> {
    let cargo = read_cargo()?;
    println!("name: {}", cargo.package.name);
    println!("version: {}", cargo.package.version);
    for (name, version) in &cargo.dependencies {
        match version {
            Dependency::Detailed { version, features } => {
                println!("{} = {}", name, version);
                if let Some(features_list) = features {
                    for feature in features_list {
                        println!("feature: {}", feature);
                    }
                }
            }
            Dependency::Simple(version) => {
                println!("{} = {}", name, version);
            }
        }
    }
    Ok(())
}
pub fn install(package: &str) -> Result<(), Box<dyn Error>> {
    let mut package_dir = PathBuf::from(format!("my_project/registry/{}", package));
    let cargo = read_cargo()?;
    let version_package = cargo.dependencies.get(package);
    let mut suitable_versions = Vec::new();
    match fs::read_dir(&package_dir) {
        Ok(entries) => {
            println!("Найдены версии:");
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let filname = path.file_name().unwrap().to_string_lossy();
                let version_base = Version::parse(&filname)?;
                print!("{}", version_base);

                if let Some(requirement) = version_package {
                    let version_cargo = match requirement {
                        Dependency::Detailed {
                            version,
                            features: _,
                        } => version,
                        Dependency::Simple(version) => version,
                    };
                    let requirement = VersionReq::parse(version_cargo)?;
                    if requirement.matches(&version_base) {
                        println!(" - подходит");
                        suitable_versions.push(version_base.clone());
                    } else {
                        println!(" - НЕ подходит");
                    }
                }
            }
            if let Some(version) = suitable_versions.iter().max() {
                println!("Подходящая версия: {}", version);
                package_dir.push(version.to_string());

                let install_dir = PathBuf::from(format!(
                    "./my_project/.mycargo/installed/{}/{}",
                    package, version
                ));
                if !fs::exists(&install_dir)? {
                    fs::create_dir_all(&install_dir)?;
                    let mut options = CopyOptions::new();
                    options.copy_inside = true;
                    options.content_only = true;
                    copy(package_dir, install_dir, &options)?;
                } else {
                    println!("Версия {} уже установлена", version);
                }
            }
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

pub fn remove(package: &str) -> Result<(), Box<dyn Error>> {
    let installed_dir = PathBuf::from(format!("my_project/.mycargo/installed/{}", package));
    if installed_dir.exists() {
        println!("Удаляю пакет {}", package);
        remove_package(installed_dir)?;
    } else {
        println!("Пакет {} не установлен", package);
    }
    Ok(())
}

pub fn search(package: &str) -> Result<(), Box<dyn Error>> {
    let package_dir = PathBuf::from(format!("my_project/registry/{}", package));
    match fs::read_dir(package_dir) {
        Ok(entries) => {
            println!("Найден пакет {package}:");
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let version = path.file_name().unwrap().to_string_lossy();
                println!("{version}");
            }
        }
        Err(e) => {
            return Err(e.into());
        }
    }
    Ok(())
}

pub fn update() -> Result<(), Box<dyn Error>> {
    let cargo = read_cargo()?;

    for (package, dependency) in &cargo.dependencies {
        // найти последнюю подходящую версию
        // сравнить с установленной
        // при необходимости установить
        let version_cargo = dependency.version();
        println!("{package}: {version_cargo}");
        let requirement = VersionReq::parse(version_cargo)?;
        let install_package_dir =
            PathBuf::from(format!("my_project/.mycargo/installed/{}", package));
        let version_install = fs::read_dir(&install_package_dir).ok().and_then(|entries| {
            entries
                .filter_map(|res| res.ok())
                .filter_map(|entry| {
                    let path = entry.path();

                    if path.is_dir() {
                        path.file_name()
                            .and_then(|name| name.to_str())
                            .and_then(|name| Version::parse(name).ok())
                    } else {
                        None
                    }
                })
                .max()
        });
        let mut package_dir = PathBuf::from(format!("my_project/registry/{}", package));
        let mut suitable_versions = Vec::new();
        match fs::read_dir(&package_dir) {
            Ok(entries) => {
                println!("Найдены версии {package}:");
                for entry in entries {
                    let entry = entry?;
                    let path = entry.path();
                    let version_dir = path.file_name().unwrap().to_string_lossy();
                    let version_base = Version::parse(&version_dir)?;
                    if requirement.matches(&version_base) {
                        suitable_versions.push(version_base.clone());
                    }
                }

                if let Some(version) = suitable_versions.iter().max() {
                    println!("Подходящая версия: {}", version);
                    package_dir.push(version.to_string());
                    let install_dir = PathBuf::from(format!(
                        "./my_project/.mycargo/installed/{}/{}",
                        package, version
                    ));
                    if let Some(ver_install) = version_install {
                        let install_cur_version = PathBuf::from(format!(
                            "./my_project/.mycargo/installed/{}/{}",
                            package, ver_install
                        ));
                        if ver_install < *version
                            && !fs::exists(&install_dir)?
                            && fs::exists(&install_cur_version)?
                        {
                            // Обновляем пакет
                            println!("Обновляем пакет с версии {ver_install} до {version}");
                            remove_package(install_cur_version)?;
                            install_version(package_dir, install_dir)?;
                        }
                    } else {
                        if !fs::exists(&install_dir)? {
                            // Устанавливаем новый пакет
                            println!("Устанавливаем версию v{version}");
                            install_version(package_dir, install_dir)?;
                        } else {
                            // Не устанавливаем, т.к. уже есть актуальный пакет
                        }
                    }
                }
            }
            Err(_) => {
                println!("Версии не найдены у пакета {package}");
            }
        }
    }

    Ok(())
}

fn install_version(package_dir: PathBuf, install_dir: PathBuf) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(&install_dir)?;

    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.content_only = true;

    copy(package_dir, install_dir, &options)?;

    Ok(())
}
