// SPDX-License-Identifier: GPL-2.0

use kernel::fs::fs_type::{FsContext, FsRegistration};
use kernel::prelude::*;
use kernel::{macros::pinned_drop, try_pin_init};

module! {
    type: Rminix,
    name: "rminix",
    author: "Sidong Yang",
    description: "Rust implementation for minix filesystem",
    license: "GPL v2",
}

#[pin_data(PinnedDrop)]
struct Rminix {
    #[pin]
    fs_type: FsRegistration<MinixFsTypeOperations>,
}

#[pinned_drop]
impl PinnedDrop for Rminix {
    fn drop(self: Pin<&mut Self>) {}
}

impl kernel::InPlaceModule for Rminix {
    fn init(module: &'static ThisModule) -> impl PinInit<Self, Error> {
        pr_info!("init rminix\n");
        try_pin_init!(Self {
            fs_type <- FsRegistration::register(module, "rminix"),
        })
    }
}

struct MinixFsTypeOperations {}

#[vtable]
impl kernel::fs::fs_type::Operations for MinixFsTypeOperations {
    fn get_tree(fc: FsContext) -> Result<()> {
        Ok(())
    }

    fn reconfigure(fc: FsContext) -> Result<()> {
        Ok(())        
    }
}

