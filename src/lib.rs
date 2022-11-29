use std::{fs, io, path::Path};

use toml::{macros::Deserialize, value::Table, Value};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("std::io::error - {0}")]
    Io(#[from] io::Error),

    #[error("toml::de::Error - {0}")]
    DeToml(#[from] toml::de::Error),

    #[error("toml::ser::Error - {0}")]
    SerToml(#[from] toml::ser::Error),
}

pub fn parse<'de, T, P>(path: P) -> Result<T, Error>
where
    T: Deserialize<'de>,
    P: AsRef<Path>,
{
    let mut dir_table = Table::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        let file_name = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let value = match file_type.is_dir() {
            true => parse(&path)?,
            false => from_file(&path)?,
        };

        dir_table.insert(file_name, value);
    }

    Ok(Value::Table(dir_table).try_into()?)
}

fn from_file<P>(path: P) -> Result<Value, Error>
where
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(path)?;
    Ok(toml::from_str(&contents)?)
}
