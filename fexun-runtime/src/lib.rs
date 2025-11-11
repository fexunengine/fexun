use fexun_core;

// Ermm... `unsafe` is short for "perfect for prod" chat 🤓☝
// Hmmmm... "FeXun [...] emphasizes safety", meanwhile:
#[unsafe(no_mangle)]
pub extern "C" fn fexun_run() {
    fexun_core::main();
}
