use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;

use crate::providers::ModSpecification;

use super::{Lint, LintCtxt};

#[derive(Default)]
pub struct ShaderFilesLint;

impl Lint for ShaderFilesLint {
    type Output = BTreeMap<ModSpecification, BTreeSet<String>>;

    fn check_mods(&mut self, lcx: &LintCtxt) -> Result<Self::Output> {
        let mut shader_file_mods = BTreeMap::new();

        lcx.for_each_mod_file(
            |mod_spec, _, raw_path, normalized_path| {
                if raw_path.extension().and_then(std::ffi::OsStr::to_str) == Some("ushaderbytecode")
                {
                    shader_file_mods
                        .entry(mod_spec)
                        .and_modify(|paths: &mut BTreeSet<String>| {
                            paths.insert(normalized_path.clone());
                        })
                        .or_insert_with(|| [normalized_path].into());
                }
                Ok(())
            },
            None::<fn(ModSpecification)>,
            None::<fn(ModSpecification)>,
        )?;

        Ok(shader_file_mods)
    }
}
