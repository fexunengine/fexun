use bytesize::ByteSize;
use pollster::block_on;
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
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter =
            block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default())).unwrap();
        let gpu = adapter.get_info();

        let mut system = System::new_all();
        system.refresh_all();

        let cpu = &system.cpus()[0];

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
        info!("CPU: \"{}\"", cpu.brand());
        info!("GPU: {:?}", gpu.name);
    } else {
        info!("System cannot be identified. Are you running FeXun on an unsupported platform?");
    }
}
