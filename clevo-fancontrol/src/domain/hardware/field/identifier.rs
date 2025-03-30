use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
    index: u32,
    desc: String,
}

impl Default for Identifier {
    fn default() -> Self {
        Self {
            index: 0,                    // Default index
            desc: "Unknown".to_string(), // Default description
        }
    }
}

impl Identifier {
    pub fn new(index: u32, desc: &str) -> Self {
        if desc.is_empty() {
            Self {
                index,
                desc: "unknow".to_string(), // Default description if empty
            }
        } else {
            Self {
                index,
                desc: desc.to_string(),
            } // Include index in the description
        }
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn get_desc(&self) -> &str {
        &self.desc
    }
}
