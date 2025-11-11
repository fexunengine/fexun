use fexun_core;

#[unsafe(no_mangle)]
pub extern "C" fn fexun_run() {
    fexun_core::main();
}
