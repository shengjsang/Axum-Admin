use sysinfo::{DiskExt, System, SystemExt};

pub fn get() {
    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // We display all disks' information:
    println!("=> disks:");

    println!(
        "total {:?} G",
        sys.disks()[0].total_space() / (1024 * 1024 * 1024)
    );

    println!("=> system:");
    // RAM and swap information:
    println!(
        "total memory: {} G",
        sys.total_memory() / (1024 * 1024 * 1024)
    );
    println!(
        "used memory : {} G",
        sys.used_memory() / (1024 * 1024 * 1024)
    );

    // Display system information:
    println!("System name:             {:?}", sys.name().unwrap());
    println!(
        "System kernel version:   {:?}",
        sys.kernel_version().unwrap()
    );
    println!("System OS version:       {:?}", sys.os_version().unwrap());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());
}
