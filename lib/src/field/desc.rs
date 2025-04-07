use bincode::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub enum ComponentType {
    Cpu,
    Gpu,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct Desc {
    component_type: ComponentType,
    index: u8,
    name: String,
}

impl Desc {
    pub fn new(component_type: ComponentType, index: u8, name: &str) -> Self {
        Self {
            component_type,
            index,
            name: name.to_string(),
        }
    }

    pub fn get_type(&self) -> &ComponentType {
        &self.component_type
    }

    pub fn get_index(&self) -> u8 {
        self.index
    }

    pub fn get_desc(&self) -> &str {
        &self.name
    }
}
