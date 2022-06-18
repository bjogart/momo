use std::path::Path;

use code::Queries as _;
use config::Config;
use name::Queries as _;
use query::QueryCx;

pub fn compile<P>(entry: P) -> wasm::Mod
where
    P: AsRef<Path>,
{
    let entry = entry.as_ref().canonicalize().unwrap();
    QueryCx::enter(Config::new(entry), |qx| qx.codegen_binary(qx.entry_func()))
}
