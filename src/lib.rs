// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate glib;
extern crate javascriptcore_sys;

use std::ptr;

use glib::translate::{FromGlibPtrFull, FromGlibPtrNone};
use javascriptcore_sys::*;

pub struct GlobalContext {
    raw: JSGlobalContextRef,
}

pub struct Value {
    raw: JSValueRef,
}

impl Value {
    pub fn is_boolean(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueIsBoolean(context.raw, self.raw) != 0 }
    }

    pub fn is_null(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueIsNull(context.raw, self.raw) != 0 }
    }

    pub fn is_undefined(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueIsUndefined(context.raw, self.raw) != 0 }
    }

    pub fn is_number(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueIsNumber(context.raw, self.raw) != 0 }
    }

    pub fn is_string(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueIsString(context.raw, self.raw) != 0 }
    }

    pub fn is_object(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueIsObject(context.raw, self.raw) != 0 }
    }

    pub fn is_array(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueIsArray(context.raw, self.raw) != 0 }
    }

    pub fn is_date(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueIsDate(context.raw, self.raw) != 0 }
    }

    pub fn to_number(&self, context: &GlobalContext) -> Option<f64> {
        let mut exception = ptr::null_mut();
        let result = unsafe { JSValueToNumber(context.raw, self.raw, &mut exception) };
        if exception.is_null() {
            Some(result)
        } else {
            None
        }
    }

    pub fn to_boolean(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueToBoolean(context.raw, self.raw) != 0}
    }

    pub fn to_string(&self, context: &GlobalContext) -> Option<String> {
        unsafe {
            let mut exception = ptr::null_mut();
            let jsstring = JSValueToStringCopy(context.raw, self.raw, &mut exception);

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

impl FromGlibPtrNone<JSValueRef> for Value {
    unsafe fn from_glib_none(ptr: JSValueRef) -> Self {
        Value {
            raw: ptr,
        }
    }
}

impl FromGlibPtrFull<JSValueRef> for Value {
    unsafe fn from_glib_full(ptr: JSValueRef) -> Self {
        Value {
            raw: ptr,
        }
    }
}

impl FromGlibPtrNone<JSGlobalContextRef> for GlobalContext {
    unsafe fn from_glib_none(ptr: JSGlobalContextRef) -> Self {
        GlobalContext {
            raw: ptr,
        }
    }
}

impl FromGlibPtrFull<JSGlobalContextRef> for GlobalContext {
    unsafe fn from_glib_full(ptr: JSGlobalContextRef) -> Self {
        GlobalContext {
            raw: ptr,
        }
    }
}
