just := env_var_or_default("JUST", just_executable())
cargo := env_var_or_default("CARGO", "cargo")
wasm_tools := env_var_or_default("WASM_TOOLS", "wasm-tools")
wash := env_var_or_default("WASH", "wash")

expected_wasm_path := "target/wasm32-wasi/release/wasmcon2023_keyvalue.wasm"
wasm_preview2_output_path := "target/wasm32-wasi/release/wasmcon2023_keyvalue.preview2.wasm"

_default:
    {{just}} --list

# Build the project (using the default methodology)
build: build-wasmcloud # build-wasm-tools

# Build the project, continuously
build-watch:
    {{cargo}} watch --ignore=target -- {{just}} build

# (build methodology) Build the WASM components using wasmcloud tooling (wash)
build-wasmcloud:
    @echo "[warning] ensure you're using a version of wash newer than 7111b5d9a5ece7543ded436b7816974ad27910e2"
    @echo "[warning] you can override the version of wash used by setting WASH"
    @{{wash}} build

# (build methodology) Build the WASM components using Bytecode Alliance tooling (wasm-tools)
build-wasm-tools:
    # Building wasm preview1 module...
    @{{cargo}} build --target=wasm32-wasi --release
    # Adapting wasm preview1 module to preview2 component...
    @{{wasm_tools}} component new --adapt=wasi_snapshot_preview1.wasm {{expected_wasm_path}} -o {{wasm_preview2_output_path}}
    @echo "[success] preview2 component output to [{{wasm_preview2_output_path}}]"


##############
# Operations #
##############

# NOTE: don't forget to set up WASMCLOUD_CLUSTER_SEED on the rust host when you start it!

expected_signed_wasm_path := absolute_path("build/wasmcon2023_keyvalue_s.wasm")

# Run the actor on a given host
run-actor host_id:
    @echo "[warn] make sure to set WASMCLOUD_CLUSTER_SEED and WASMCLOUD_CLUSTER_ISSUERS appropriately for new hosts"
    @echo "(see output of wash up for expected values of cluster seed & issuers)"
    @{{wash}} start actor file://{{expected_signed_wasm_path}} --host-id {{host_id}}

# Set up links to a given in-lattice actor_id
setup-links actor_id:
    # Setting up links for HTTP provider (VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M)
    @{{wash}} link put {{actor_id}} VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M wasmcloud:httpserver
    # Setting up links for kvredis provider (VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M)
    @{{wash}} link put {{actor_id}} VAZVC4RX54J2NVCMCW7BPCAHGGG5XZXDBXFUMDUXGESTMQEJLC3YVZWB wasmcloud:keyvalue
