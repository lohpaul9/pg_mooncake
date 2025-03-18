MAKEFLAGS := --no-print-directory

run:
	@$(MAKE) -C pg_duckdb duckdb all-static-lib DUCKDB_BUILD=ReleaseStatic DUCKDB_GEN=make
	@cargo pgrx run

clean:
	@$(MAKE) -C pg_duckdb clean-all
	@cargo clean

format:
	@$(MAKE) -C pg_duckdb format-all
	@cargo fmt
