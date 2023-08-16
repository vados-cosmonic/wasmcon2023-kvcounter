just := env_var_or_default("JUST", just_executable())
cargo := env_var_or_default("CARGO", "cargo")
wasm_tools := env_var_or_default("WASM_TOOLS", "wasm-tools")

expected_wasm_path := "target/wasm32-wasi/release/wasmcon2023_keyvalue.wasm"

_default:
    {{just}} --list

# Build the WASM component
build:
    {{cargo}} build --target=wasm32-wasi --release
    {{wasm_tools}} component new --adapt=wasi_snapshot_preview1.wasm {{expected_wasm_path}}

# Run the demo
run:
    {{cargo}} run --bin wasm-component-demo --manifest-path=wasm-component-demo/Cargo.toml
