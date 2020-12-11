use serde::{Deserialize, Serialize};

#[derive(std::fmt::Debug, Serialize, Deserialize, Clone)]
pub struct Setting {
    table_path: String,
}

impl Setting {
    pub fn table_path(&self) -> String {
        self.table_path.clone()
    }
}
