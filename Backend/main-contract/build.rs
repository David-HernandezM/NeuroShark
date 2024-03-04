/*
use gear_wasm_builder::WasmBuilder;
use program_io::ProgramMetadata;
use gmeta::Metadata;

fn main() {
    WasmBuilder::with_meta(ProgramMetadata::repr())
        .exclude_features(["binary-vendor"])
        .build();
}
*/

use main_contract_io::ProgramMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<ProgramMetadata>();
}




//hola namin cara de tapoyi