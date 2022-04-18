use std::path::{Path, PathBuf};
use crate::types::chain::ChainId;

pub struct ParamsDirectory {
    pub path: PathBuf,
}

impl ParamsDirectory {
    pub fn for_chain_directory(chain_dir: impl AsRef<Path>) -> Self {
        ParamsDirectory { path: chain_dir.as_ref().join("masp") }
    }

    pub fn spend_path(&self) -> PathBuf {
        (&self).path.join("masp-spend.params")
    }

    pub fn output_path(&self) -> PathBuf {
        (&self).path.join("masp-output.params")
    }
}

impl AsRef<Path> for ParamsDirectory {
    fn as_ref(&self) -> &Path {
        return &self.path;
    }
}
