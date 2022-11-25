use ffi::*;
use glib::translate::*;

pub struct GlobalContextRef {
  raw: JSGlobalContextRef,
}

impl FromGlibPtrNone<JSGlobalContextRef> for GlobalContextRef {
  unsafe fn from_glib_none(ptr: JSGlobalContextRef) -> Self {
    GlobalContextRef { raw: ptr }
  }
}

impl FromGlibPtrFull<JSGlobalContextRef> for GlobalContextRef {
  unsafe fn from_glib_full(ptr: JSGlobalContextRef) -> Self {
    GlobalContextRef { raw: ptr }
  }
}

impl<'a> ToGlibPtr<'a, JSGlobalContextRef> for GlobalContextRef {
  type Storage = ();

  #[inline]
  fn to_glib_none(&self) -> Stash<'a, JSGlobalContextRef, GlobalContextRef> {
    Stash(self.raw, ())
  }
}
