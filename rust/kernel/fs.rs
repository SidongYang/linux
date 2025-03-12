// SPDX-License-Identifier: GPL-2.0

//! Kernel file systems.
//!
//! C headers: [`include/linux/fs.h`](srctree/include/linux/fs.h)

pub mod file;
pub mod fs_type;
pub mod fs_context;
pub mod super_block;
pub use self::file::{File, LocalFile};
