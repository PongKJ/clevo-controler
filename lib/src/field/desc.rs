use super::category::{self, Category};
use bincode::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub struct Desc {
    category: Category,
    index: u8,
    name: String,
}

impl Desc {
    pub fn new(category: Category, index: u8, name: &str) -> Self {
        Self {
            category,
            index,
            name: name.to_string(),
        }
    }

    pub fn get_category(&self) -> &Category {
        &self.category
    }

    pub fn get_index(&self) -> u8 {
        self.index
    }

    pub fn get_desc(&self) -> &str {
        &self.name
    }
}
