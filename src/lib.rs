mod mooncakeam;

use pgrx::prelude::*;

pgrx::pg_module_magic!();
pgrx::extension_sql_file!("./sql/bootstrap.sql", bootstrap);

extern "C" {
    fn init_pg_duckdb();
    fn execute_duckdb_query(query: *const i8) -> *const i8;
}

#[pg_guard]
pub extern "C" fn _PG_init() {
    unsafe { init_pg_duckdb() };
}

#[pg_extern]
fn start_duckdb_ui() -> String {
    let query = "CALL start_ui();";
    unsafe {
        let c_query = std::ffi::CString::new(query).unwrap();
        let result = execute_duckdb_query(c_query.as_ptr());
        let c_str = std::ffi::CStr::from_ptr(result);
        c_str.to_string_lossy().into_owned()
    }
}