use std::fs;

use anyhow::Result;

use crate::db::DBState;

pub trait Database {
    fn read_file(&self) -> Result<DBState>;
    fn write_file(&self, db_state: &DBState) -> Result<()>;
}

#[derive(Clone)]
pub struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_file(&self) -> Result<DBState> {
        let file_str_data = fs::read_to_string(self.file_path.clone())?;

        let db_state: DBState = serde_json::from_str(&file_str_data)?;

        Ok(db_state)
    }

    fn write_file(&self, db_state: &DBState) -> Result<()> {
        let json = serde_json::to_vec(db_state)?;
        fs::write(self.file_path.clone(), json)?;

        Ok(())
    }
}
