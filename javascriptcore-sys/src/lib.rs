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

extern crate libc;

use libc::{c_double, c_void};

pub type JSGlobalContextRef = *mut c_void;
pub type JSValueRef = *mut c_void;

extern "C" {
    pub fn JSValueIsBoolean(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueIsNull(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueIsUndefined(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueIsNumber(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueIsString(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueIsObject(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueIsArray(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueIsDate(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueToNumber(ctx: JSGlobalContextRef, value: JSValueRef, exception: *mut JSValueRef) -> c_double;
    pub fn JSValueToBoolean(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
}
