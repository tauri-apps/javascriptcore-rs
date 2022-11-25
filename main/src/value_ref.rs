// // Copyright 2013-2017, The Gtk-rs Project Developers.
// // See the COPYRIGHT file at the top-level directory of this distribution.
// // Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>
#![cfg_attr(feature = "dox", feature(doc_cfg))]

use ffi::*;
use glib::translate::*;
use std::ptr;

use crate::GlobalContextRef;

pub struct ValueRef {
  raw: JSValueRef,
}

impl ValueRef {
  pub fn is_boolean(&self, context: &GlobalContextRef) -> bool {
    unsafe { JSValueIsBoolean(context.to_glib_none().0, self.raw) != 0 }
  }

  pub fn is_null(&self, context: &GlobalContextRef) -> bool {
    unsafe { JSValueIsNull(context.to_glib_none().0, self.raw) != 0 }
  }

  pub fn is_undefined(&self, context: &GlobalContextRef) -> bool {
    unsafe { JSValueIsUndefined(context.to_glib_none().0, self.raw) != 0 }
  }

  pub fn is_number(&self, context: &GlobalContextRef) -> bool {
    unsafe { JSValueIsNumber(context.to_glib_none().0, self.raw) != 0 }
  }

  pub fn is_string(&self, context: &GlobalContextRef) -> bool {
    unsafe { JSValueIsString(context.to_glib_none().0, self.raw) != 0 }
  }

  pub fn is_object(&self, context: &GlobalContextRef) -> bool {
    unsafe { JSValueIsObject(context.to_glib_none().0, self.raw) != 0 }
  }

  pub fn is_array(&self, context: &GlobalContextRef) -> bool {
    unsafe { JSValueIsArray(context.to_glib_none().0, self.raw) != 0 }
  }

  pub fn is_date(&self, context: &GlobalContextRef) -> bool {
    unsafe { JSValueIsDate(context.to_glib_none().0, self.raw) != 0 }
  }

  pub fn to_number(&self, context: &GlobalContextRef) -> Option<f64> {
    let mut exception = ptr::null_mut();
    let result = unsafe { JSValueToNumber(context.to_glib_none().0, self.raw, &mut exception) };
    if exception.is_null() {
      Some(result)
    } else {
      None
    }
  }

  pub fn to_boolean(&self, context: &GlobalContextRef) -> bool {
    unsafe { JSValueToBoolean(context.to_glib_none().0, self.raw) != 0 }
  }

  pub fn to_string(&self, context: &GlobalContextRef) -> Option<String> {
    unsafe {
      let mut exception = ptr::null_mut();
      let jsstring = JSValueToStringCopy(context.to_glib_none().0, self.raw, &mut exception);

      if exception.is_null() {
        let cap = JSStringGetMaximumUTF8CStringSize(jsstring);
        let mut buf = Vec::<u8>::with_capacity(cap);
        let len = JSStringGetUTF8CString(jsstring, buf.as_mut_ptr() as _, cap);
        JSStringRelease(jsstring);
        buf.set_len(len - 1);
        String::from_utf8(buf).ok()
      } else {
        None
      }
    }
  }
}

impl FromGlibPtrNone<JSValueRef> for ValueRef {
  unsafe fn from_glib_none(ptr: JSValueRef) -> Self {
    ValueRef { raw: ptr }
  }
}

impl FromGlibPtrFull<JSValueRef> for ValueRef {
  unsafe fn from_glib_full(ptr: JSValueRef) -> Self {
    ValueRef { raw: ptr }
  }
}

impl<'a> ToGlibPtr<'a, JSValueRef> for ValueRef {
  type Storage = ();

  #[inline]
  fn to_glib_none(&self) -> Stash<'a, JSValueRef, ValueRef> {
    Stash(self.raw, ())
  }
}
