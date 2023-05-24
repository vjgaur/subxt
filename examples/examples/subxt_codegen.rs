use std::fs;
use codec::Decode;

use frame_metadata::RuntimeMetadataPrefixed;
use subxt_codegen::{CratePath, DerivesRegistry, TypeSubstitutes};

fn main() {
    let encoded = fs::read("../artifacts/substrate.scale").unwrap();

    // Runtime metadata obtained from a node.
    let metadata = <RuntimeMetadataPrefixed as Decode>::decode(&mut &*encoded).unwrap();
    // Module under which the API is generated.
    let item_mod = syn::parse_quote!(
        pub mod api {}
    );
    // Default module derivatives.
    let mut derives = DerivesRegistry::new(&CratePath::default());
    // Default type substitutes.
    let substs = TypeSubstitutes::new(&CratePath::default());
    // Generate the Runtime API.
    let generator = subxt_codegen::RuntimeGenerator::new(metadata);
    // Include metadata documentation in the Runtime API.
    let generate_docs = true;
    let runtime_api = generator.generate_runtime(item_mod, derives, substs, CratePath::default(), generate_docs).unwrap();
    println!("{}", runtime_api);
}
