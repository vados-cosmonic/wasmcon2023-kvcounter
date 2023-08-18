just := env_var_or_default("JUST", just_executable())
cargo := env_var_or_default("CARGO", "cargo")
wasm_tools := env_var_or_default("WASM_TOOLS", "wasm-tools")

expected_wasm_path := "target/wasm32-wasi/release/wasmcon2023_keyvalue.wasm"
wasm_preview2_output_path := "target/wasm32-wasi/release/wasmcon2023_keyvalue.preview2.wasm"

_default:
    {{just}} --list

# Ensure wasm-tools is the "right" version, for now.
# we have to use the latest of essentially *everything* to get this to work
check-wasm-tools:
    @echo "[warning] you must use a build a custom version of wasm-tools and comment out the record type field check!"
    @echo "search for 'record type must have at least one field' in wasmparser/src/validator/component.rs"
    @({{wasm_tools}} --version | grep "wasm-tools 1.0.38 (a0c46a7a1 2023-08-10)") || (echo "ERROR: please locally build & run latest wasm-tools @ a0c46a7a1)" && exit -1)

# Build the WASM component
build: check-wasm-tools
    # Building wasm preview1 module...
    @{{cargo}} build --target=wasm32-wasi --release
    # Adapting wasm preview1 module to preview2 component...
    @{{wasm_tools}} component new --adapt=wasi_snapshot_preview1.wasm {{expected_wasm_path}} -o {{wasm_preview2_output_path}}
    @echo "[success] preview2 component output to [{{wasm_preview2_output_path}}]"
