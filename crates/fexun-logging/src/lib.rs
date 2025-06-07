use bytesize::ByteSize;
use sysinfo::System;
use tracing::{self, info};
use tracing_subscriber::{EnvFilter, fmt};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn start_logging() {
    let filter = EnvFilter::new(
        "info,\
         wgpu_hal=warn,\
         wgpu_core=warn,\
         fexun=debug",
    );
    fmt().with_env_filter(filter).init();

    if sysinfo::IS_SUPPORTED_SYSTEM {
        let mut system = System::new_all();
        system.refresh_all();
        info!("Running FeXun Version {}", VERSION);
        info!(
            "System Name: {:?}",
            System::name().unwrap_or("Unknown".to_string())
        );
        info!(
            "System Version: {:?}",
            System::os_version().unwrap_or("Unknown".to_string())
        );
        info!("Kernel Version: {:?}", System::kernel_long_version());
        info!(
            "Available memory: {}\n",
            ByteSize::b(system.total_memory() as u64)
        );
    } else {
        info!("System cannot be identified. Are you running FeXun on an unsupported platform?");
    }
}
