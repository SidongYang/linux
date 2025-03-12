use core::ffi;
use core::marker::PhantomData;
use kernel::error;
use kernel::error::Error;
use kernel::{prelude::*, types::Opaque};

#[repr(transparent)]
#[pin_data(PinnedDrop)]
pub struct FsRegistration<T: Operations> {
    #[pin]
    inner: Opaque<bindings::file_system_type>,
    _p: PhantomData<T>,
}

#[macros::vtable]
pub trait Operations {
    fn get_tree(fc: FsContext) -> Result {
        build_error!(error::VTABLE_DEFAULT_ERROR)
    }

    fn reconfigure(fc: FsContext) -> Result {
        build_error!(error::VTABLE_DEFAULT_ERROR)
    }
}

unsafe impl<T: Operations + Send> Send for FsRegistration<T> {}
unsafe impl<T: Operations + Sync> Sync for FsRegistration<T> {}

impl<T: Operations> FsRegistration<T> {
    pub fn register(owner: &'static ThisModule, name: &'static str) -> impl PinInit<Self, Error> {
        let mut fs_type = bindings::file_system_type::default();
        fs_type.name = name.as_ptr();
        fs_type.owner = owner.0;
        fs_type.init_fs_context = Some(OperationsVTable::<T>::init_fs_context_callback);

        try_pin_init!(Self {
            inner <- PinInit::<_, Error>::pin_chain(
                Opaque::new(fs_type), |pinned| {
                    kernel::error::to_result(
                        unsafe {
                            bindings::register_filesystem(pinned.get())
                        }
                    )}),
            _p: PhantomData::default(),
        })
    }
}

#[pinned_drop]
impl<T: Operations> PinnedDrop for FsRegistration<T> {
    fn drop(self: Pin<&mut Self>) {
        unsafe {
            bindings::unregister_filesystem(self.inner.get());
        }
    }
}

pub struct OperationsVTable<T: Operations>(PhantomData<T>);

impl<T: Operations> OperationsVTable<T> {
    const OPS_VTABLE: bindings::fs_context_operations = bindings::fs_context_operations {
        free: None,
        dup: None,
        parse_param: None,
        parse_monolithic: None,
        get_tree: Some(Self::get_tree_callback),
        reconfigure: Some(Self::reconfigure_callback),
    };
    unsafe extern "C" fn init_fs_context_callback(
        fs_context: *mut bindings::fs_context,
    ) -> core::ffi::c_int {
        unsafe {
            (*fs_context).ops = &Self::OPS_VTABLE;
        }
        0
    }

    unsafe extern "C" fn get_tree_callback(fc: *mut bindings::fs_context) -> ffi::c_int {
        let ret = T::get_tree(fc.into());
        if let Err(e) = ret {
            e.to_errno()
        } else {
            0
        }
    }

    unsafe extern "C" fn reconfigure_callback(fc: *mut bindings::fs_context) -> ffi::c_int {
        let ret = T::reconfigure(fc.into());
        if let Err(e) = ret {
            e.to_errno()
        } else {
            0
        }
    }
}

pub struct FsContext {
    inner: *mut bindings::fs_context,
}

impl From<*mut bindings::fs_context> for FsContext {
    fn from(inner: *mut bindings::fs_context) -> Self {
        Self { inner }
    }
}
