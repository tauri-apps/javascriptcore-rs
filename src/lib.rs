/*
 * Copyright (c) 2016 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

extern crate glib;
extern crate javascriptcore_sys;

use std::ptr;

use glib::translate::{FromGlibPtrFull, FromGlibPtrNone};
use javascriptcore_sys::{JSGlobalContextRef, JSValueRef, JSValueIsArray, JSValueIsDate, JSValueIsBoolean, JSValueIsNull, JSValueIsNumber, JSValueIsObject, JSValueIsString, JSValueIsUndefined, JSValueToBoolean, JSValueToNumber};

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
        }
        else {
            None
        }
    }

    pub fn to_boolean(&self, context: &GlobalContext) -> bool {
        let value = unsafe { JSValueToBoolean(context.raw, self.raw) };
        value != 0
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
    unsafe fn from_glib_none(ptr: JSValueRef) -> Self {
        GlobalContext {
            raw: ptr,
        }
    }
}

impl FromGlibPtrFull<JSGlobalContextRef> for GlobalContext {
    unsafe fn from_glib_full(ptr: JSValueRef) -> Self {
        GlobalContext {
            raw: ptr,
        }
    }
}
