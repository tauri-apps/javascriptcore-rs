// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Context;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "JSCException")]
    pub struct Exception(Object<ffi::JSCException, ffi::JSCExceptionClass>);

    match fn {
        type_ => || ffi::jsc_exception_get_type(),
    }
}

impl Exception {
    #[doc(alias = "jsc_exception_new")]
    pub fn new<P: IsA<Context>>(context: &P, message: &str) -> Exception {
        unsafe {
            from_glib_full(ffi::jsc_exception_new(context.as_ref().to_glib_none().0, message.to_glib_none().0))
        }
    }

    //#[doc(alias = "jsc_exception_new_printf")]
    //pub fn new_printf<P: IsA<Context>>(context: &P, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Exception {
    //    unsafe { TODO: call ffi:jsc_exception_new_printf() }
    //}

    //#[doc(alias = "jsc_exception_new_vprintf")]
    //pub fn new_vprintf<P: IsA<Context>>(context: &P, format: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Exception {
    //    unsafe { TODO: call ffi:jsc_exception_new_vprintf() }
    //}

    #[doc(alias = "jsc_exception_new_with_name")]
    #[doc(alias = "new_with_name")]
    pub fn with_name<P: IsA<Context>>(context: &P, name: &str, message: &str) -> Exception {
        unsafe {
            from_glib_full(ffi::jsc_exception_new_with_name(context.as_ref().to_glib_none().0, name.to_glib_none().0, message.to_glib_none().0))
        }
    }

    //#[doc(alias = "jsc_exception_new_with_name_printf")]
    //#[doc(alias = "new_with_name_printf")]
    //pub fn with_name_printf<P: IsA<Context>>(context: &P, name: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Exception {
    //    unsafe { TODO: call ffi:jsc_exception_new_with_name_printf() }
    //}

    //#[doc(alias = "jsc_exception_new_with_name_vprintf")]
    //#[doc(alias = "new_with_name_vprintf")]
    //pub fn with_name_vprintf<P: IsA<Context>>(context: &P, name: &str, format: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Exception {
    //    unsafe { TODO: call ffi:jsc_exception_new_with_name_vprintf() }
    //}
}

impl fmt::Display for Exception {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&ExceptionExt::to_str(self))
    }
}

pub const NONE_EXCEPTION: Option<&Exception> = None;

pub trait ExceptionExt: 'static {
    #[doc(alias = "jsc_exception_get_backtrace_string")]
    #[doc(alias = "get_backtrace_string")]
    fn backtrace_string(&self) -> Option<glib::GString>;

    #[doc(alias = "jsc_exception_get_column_number")]
    #[doc(alias = "get_column_number")]
    fn column_number(&self) -> u32;

    #[doc(alias = "jsc_exception_get_line_number")]
    #[doc(alias = "get_line_number")]
    fn line_number(&self) -> u32;

    #[doc(alias = "jsc_exception_get_message")]
    #[doc(alias = "get_message")]
    fn message(&self) -> Option<glib::GString>;

    #[doc(alias = "jsc_exception_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString>;

    #[doc(alias = "jsc_exception_get_source_uri")]
    #[doc(alias = "get_source_uri")]
    fn source_uri(&self) -> Option<glib::GString>;

    #[doc(alias = "jsc_exception_report")]
    fn report(&self) -> Option<glib::GString>;

    #[doc(alias = "jsc_exception_to_string")]
    #[doc(alias = "to_string")]
    fn to_str(&self) -> glib::GString;
}

impl<O: IsA<Exception>> ExceptionExt for O {
    fn backtrace_string(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::jsc_exception_get_backtrace_string(self.as_ref().to_glib_none().0))
        }
    }

    fn column_number(&self) -> u32 {
        unsafe {
            ffi::jsc_exception_get_column_number(self.as_ref().to_glib_none().0)
        }
    }

    fn line_number(&self) -> u32 {
        unsafe {
            ffi::jsc_exception_get_line_number(self.as_ref().to_glib_none().0)
        }
    }

    fn message(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::jsc_exception_get_message(self.as_ref().to_glib_none().0))
        }
    }

    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::jsc_exception_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn source_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::jsc_exception_get_source_uri(self.as_ref().to_glib_none().0))
        }
    }

    fn report(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::jsc_exception_report(self.as_ref().to_glib_none().0))
        }
    }

    fn to_str(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::jsc_exception_to_string(self.as_ref().to_glib_none().0))
        }
    }
}
