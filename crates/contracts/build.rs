// use ethers::prelude::Abigen;
// use ethers_solc::{Project, ProjectPathsConfig};
// use std::fs;
// use std::path::PathBuf;

fn main() {
    // let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    // let ext_path = root.join("ext");
    // let abi_target_path = ext_path.join("abi");
    //
    // let build_path_cfg = ProjectPathsConfig::builder()
    //     .sources(ext_path.join("contracts").join("interfaces"))
    //     .artifacts(&abi_target_path)
    //     .root(root)
    //     .build()
    //     .unwrap();
    //
    // let project = Project::builder().paths(build_path_cfg).build().unwrap();
    // let compiled = project.compile().unwrap();
    // assert!(!compiled.has_compiler_errors());
    //
    // fs::read_dir(&abi_target_path).unwrap().for_each(|dir| {
    //     if let Ok(abi) = dir {
    //         fs::read_dir(abi.path()).unwrap().for_each(|abi| {
    //             if let Ok(a) = abi {
    //                 let abi_name = a.file_name();
    //                 if let Some(file_name) = abi_name.to_str() {
    //                     let binding_name = file_name.replace(".json", ".rs");
    //                     Abigen::from_file(a.path())
    //                         .unwrap()
    //                         .generate()
    //                         .unwrap()
    //                         .write_to_file(
    //                             PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    //                                 .join(format!("src/ethereum/{}", binding_name)),
    //                         )
    //                         .unwrap();
    //                 }
    //             }
    //         })
    //     }
    // });
}
