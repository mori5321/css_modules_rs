use serde::{Deserialize, Serialize};

#[derive(std::fmt::Debug, Serialize, Deserialize)]
pub struct CorrelationTable(Vec<Correlation>);

#[derive(std::fmt::Debug, Serialize, Deserialize)]
pub struct Correlation {
    filepath: String,
    prefix: String,
}

impl CorrelationTable {
    pub fn new() -> Self {
        let correlations: Vec<Correlation> = vec![];
        CorrelationTable(correlations)
    }

    // TODO: throw Error when duplicated Correlation appended.
    pub fn append(&mut self, filepath: String, prefix: String) -> () {
        let correlation = Correlation { filepath, prefix };
        self.0.push(correlation);
    }

    pub fn search_hash(&self, filepath: String) -> Option<String> {
        let correlation = self
            .0
            .iter()
            .find(|correlation| correlation.filepath == filepath);

        match correlation {
            None => None,
            Some(correl) => Some(correl.prefix.clone()),
        }
    }
}
