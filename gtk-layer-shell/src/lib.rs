#![cfg_attr(feature = "dox", feature(doc_cfg))]

macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    };
}

/// No-op.
macro_rules! skip_assert_initialized {
    () => {};
}

mod auto;

#[allow(unused_imports)]
use gtk; // Required for the documentation to build without warnings

pub use self::auto::functions::*;
pub use auto::*;
