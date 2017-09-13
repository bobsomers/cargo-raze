use cargo::CargoError;
use cargo::core::Dependency;
use cargo::core::TargetKind;
use cargo::core::dependency::Kind;
use cargo::util::CargoResult;
use cargo::util::Cfg;
use std::collections::HashSet;
use std::process::Command;
use std::str::FromStr;
use std::str;

/** Extracts the dependencies that are of the provided kind. */
pub fn take_kinded_dep_names(platform_deps: &Vec<Dependency>, kind: Kind) -> HashSet<String> {
  platform_deps
    .iter()
    .filter(|d| d.kind() == kind)
    .map(|dep| dep.name().to_owned())
    .collect()
}

/**
 * Extracts consistently named Strings for the provided TargetKind.
 *
 * TODO(acmcarther): Remove this shim borrowed from Cargo when Cargo is upgraded
 */
pub fn kind_to_kinds(kind: &TargetKind) -> Vec<String> {
    match kind {
        &TargetKind::Lib(ref kinds) => kinds.iter().map(|k| k.crate_type().to_owned()).collect(),
        &TargetKind::Bin => vec!["bin".to_owned()],
        &TargetKind::ExampleBin | &TargetKind::ExampleLib(_) => vec!["example".to_owned()],
        &TargetKind::Test => vec!["test".to_owned()],
        &TargetKind::CustomBuild => vec!["custom-build".to_owned()],
        &TargetKind::Bench => vec!["bench".to_owned()],
    }
}

/**
 * Gets the proper system attributes for the provided platform triple using rustc.
 */
pub fn fetch_attrs(target: &str) -> CargoResult<Vec<Cfg>> {
    let args = vec![
      format!("--target={}", target),
      "--print=cfg".to_owned(),
    ];


    let output = try!(Command::new("rustc")
        .args(&args)
        .output()
        .map_err(|_| CargoError::from(format!("could not run rustc to fetch attrs for target {}", target))));

    if !output.status.success() {
      panic!(format!("getting target attrs for {} failed with status: '{}' \n\
                     stdout: {}\n\
                     stderr: {}",
                     target,
                     output.status,
                     String::from_utf8(output.stdout).unwrap_or("[unparseable bytes]".to_owned()),
                     String::from_utf8(output.stderr).unwrap_or("[unparseable bytes]".to_owned())))
    }

    let attr_str = String::from_utf8(output.stdout)
        .expect("successful run of rustc's output to be utf8");

    Ok(attr_str.lines()
        .map(Cfg::from_str)
        .map(|cfg| cfg.expect("attrs from rustc should be parsable into Cargo Cfg"))
        .collect())
}

