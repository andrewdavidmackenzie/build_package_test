
// Nothing inline in this lib.rs files refers to the module in module directory

// If it is included it will be because it is discovered by the build script and included
// in generated.rs
include!(concat!(env!("OUT_DIR"), "/generated.rs"));