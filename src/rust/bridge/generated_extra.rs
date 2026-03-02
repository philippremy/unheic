//! Helper module for implementing traits on FFI types
//!
//! Usually an implementation goes here if the opaque type needs to run the
//! bindgen destruct() function to execute a C++ destructor of a type properly.
use crate::bridge::generated::UnHEICCXXImageHandle;

// This is required because this opaque type requires a drop implementation
// to avoid memory leaks. See the C++ type description.
impl Drop for UnHEICCXXImageHandle {
    fn drop(&mut self) {
        unsafe { self.destruct() };
    }
}
