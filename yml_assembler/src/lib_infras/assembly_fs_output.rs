use std::path::PathBuf;

use crate::{adapters::AssemblyOutputPort, utils::result::AppResult};

pub struct AssemblyFSOutput {
    context: PathBuf,
}

impl AssemblyFSOutput {
    pub fn new(path: PathBuf) -> Self {
        AssemblyFSOutput { context: path }
    }
}

impl AssemblyOutputPort for AssemblyFSOutput {
    fn output(&self, value: &serde_yaml::Value, file_path: &PathBuf) -> AppResult<()> {
        let outfile_path = PathBuf::from(&self.context)
            .join(file_path)
            .with_extension("yml");

        let outfile_parent = outfile_path.parent().ok_or_else(|| {
            anyhow::anyhow!(format!(
                "Could not get parent directory of {}",
                outfile_path.display()
            ))
        })?;

        if !outfile_parent.exists() {
            std::fs::create_dir_all(&outfile_parent).map_err(|e| {
                anyhow::anyhow!(format!("Could not create output directory: {}", e))
            })?;
        }

        std::fs::write(
            outfile_path,
            serde_yaml::to_string(value)
                .map_err(|e| anyhow::anyhow!(format!("Could not serialize file: {}", e)))?,
        )
        .map_err(|e| anyhow::anyhow!(format!("Could not write file to output directory: {}", e)))?;

        Ok(())
    }
}
