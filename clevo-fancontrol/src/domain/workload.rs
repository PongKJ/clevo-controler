use serde::{Deserialize, Serialize};
// Hardware to monitor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workload {
    pub cpu: (u32, bool),
    pub gpu: (u32, bool), // (index, enable)
}

impl Default for Workload {
    fn default() -> Self {
        Workload {
            cpu: (0, true),  // Default to monitor CPU
            gpu: (1, false), // Default to not monitor GPU
        }
    }
}
