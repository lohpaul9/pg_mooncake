mod mooncakeam;

use pgrx::prelude::*;

pgrx::pg_module_magic!();
pgrx::extension_sql_file!("./sql/bootstrap.sql", bootstrap);

extern "C" {
    fn init_pg_duckdb();
}

#[pg_guard]
pub extern "C" fn _PG_init() {
    unsafe { init_pg_duckdb() };
}
