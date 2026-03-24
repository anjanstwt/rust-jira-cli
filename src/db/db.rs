use std::{collections::HashMap, fs};

use anyhow::Result;
use serde_json::Value;

use crate::db::DBState;

trait Database {
    fn read_file(&self) -> Result<DBState>;
    fn write_file(&self, db_state: &DBState) -> Result<()>;
}

#[derive(Clone)]
struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_file(&self) -> Result<DBState> {
        let file_str_data = fs::read_to_string(self.file_path.clone())?;

        let db_state: DBState = serde_json::from_str(&file_str_data)?;

        Ok(db_state)
    }

    fn write_file(&self, db_state: &DBState) -> Result<()> {
        Ok(())
    }
}
