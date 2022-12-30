use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Info {
    pub total_disk: u64,
    pub unused_disk: u64,
    pub total_memory: u64,
    pub unused_memory: u64,
    pub system_type: Option<String>,
    pub cpu_number: usize,
}
