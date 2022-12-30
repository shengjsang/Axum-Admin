use anyhow::Result;
use model::system::info::Info;
use sysinfo::{DiskExt, System, SystemExt};

const BYTE_GB: u64 = 1024 * 1024 * 1024;

pub fn get() -> Result<Info> {
    let mut sys = System::new_all();

    sys.refresh_all();

    Ok(Info {
        total_disk: sys.disks()[0].total_space() / BYTE_GB,
        unused_disk: sys.disks()[0].available_space() / BYTE_GB,
        total_memory: sys.total_memory() / BYTE_GB,
        unused_memory: sys.available_memory() / BYTE_GB,
        system_type: sys.name(),
        cpu_number: sys.cpus().len(),
    })
}
