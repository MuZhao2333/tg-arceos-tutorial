//! RAM filesystem used by [ArceOS](https://github.com/arceos-org/arceos).
//!
//! The implementation is based on [`axfs_vfs`].

#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod dir;
mod file;

#[cfg(test)]
mod tests;

pub use self::dir::DirNode;
pub use self::file::FileNode;

use alloc::sync::Arc;
use axfs_vfs::{VfsNodeOps, VfsNodeRef, VfsOps, VfsResult};
use spin::once::Once;

/// A RAM filesystem that implements [`axfs_vfs::VfsOps`].
pub struct RamFileSystem {
    parent: Once<VfsNodeRef>,
    root: Arc<DirNode>,
}

impl RamFileSystem {
    /// Create a new instance.
    pub fn new() -> Self {
        Self {
            parent: Once::new(),
            root: DirNode::new(None),
        }
    }

    /// Returns the root directory node in [`Arc<DirNode>`](DirNode).
    pub fn root_dir_node(&self) -> Arc<DirNode> {
        self.root.clone()
    }

    /// Renames a file or directory from `src_path` to `dst_path`.
    pub fn rename(&self, src_path: &str, dst_path: &str) -> VfsResult {
        log::info!("RamFileSystem::rename: src={} dst={}", src_path, dst_path);
        VfsNodeOps::rename(&*self.root, src_path, dst_path)
    }
}

impl VfsOps for RamFileSystem {
    fn mount(&self, _path: &str, mount_point: VfsNodeRef) -> VfsResult {
        if let Some(parent) = mount_point.parent() {
            self.root.set_parent(Some(self.parent.call_once(|| parent)));
        } else {
            self.root.set_parent(None);
        }
        Ok(())
    }

    fn root_dir(&self) -> VfsNodeRef {
        self.root.clone()
    }
}

impl Default for RamFileSystem {
    fn default() -> Self {
        Self::new()
    }
}
