use std::path::PathBuf;
use ethers_solc::{Solc, Project, ProjectPathsConfig, ProjectBuilder};
use ethers_solc::artifacts::Sources;
use ethers_solc::project::ProjectCompiler;

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../ext");
    let target_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/abi");
    let build_path_cfg = ProjectPathsConfig::builder()
        .sources(root.join("contracts").join("interfaces"))
        .artifacts(target_path)
        .root(root)
        .build()
        .unwrap();

    let project = Project::builder().paths(build_path_cfg).build().unwrap();
    // project.rerun_if_sources_changed();
    let compiled = project.compile().unwrap();
    assert!(!compiled.has_compiler_errors())
}

