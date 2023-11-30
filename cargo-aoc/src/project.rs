use aoc_runner_internal::DayParts;
use std::error;
use std::fs;
use std::process;
use crate::errors::CouldNotLoadDayParts;

#[derive(Clone, Debug)]
pub struct ProjectManager {
    pub name: String,
    pub slug: String,
    pub lib_path: Option<String>,
    pub lib_name: Option<String>,
}

impl ProjectManager {
    pub fn new() -> Result<ProjectManager, Box<dyn error::Error>> {
        let cargo: toml::Value = fs::read_to_string("Cargo.toml")?.parse()?;

        let crate_name = cargo
            .get("package")
            .ok_or("no field package in Cargo.toml")?
            .get("name")
            .ok_or("no field package.name in Cargo.toml")?
            .as_str()
            .ok_or("invalid crate name")?
            .to_string();

        let crate_slug = crate_name.replace('-', "_");

        let lib_path = cargo
            .get("lib")
            .and_then(|lib| lib.get("path"))
            .and_then(|lib_path| lib_path.as_str())
            .map(String::from);

        let lib_name = cargo
            .get("lib")
            .and_then(|lib| lib.get("name"))
            .and_then(|lib_name| lib_name.as_str())
            .map(String::from)
            .map(|lib_name| lib_name.replace('-', "_"));

        Ok(ProjectManager {
            name: crate_name,
            slug: crate_slug,
            lib_path,
            lib_name,
        })
    }

    pub fn build_project(&self) -> Result<DayParts, Box<dyn error::Error>> {
        let args = vec!["check", "--color=always"];

        let status = process::Command::new("cargo").args(&args).spawn()?.wait()?;

        if !status.success() {
            return Err(format!(
                "cargo build failed with code {}",
                status.code().unwrap_or(-1)
            )
            .into());
        }

        DayParts::load().map_err(|err| CouldNotLoadDayParts(err).into())
    }
}
