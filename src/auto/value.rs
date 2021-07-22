// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Context;
use crate::ValuePropertyFlags;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "JSCValue")]
    pub struct Value(Object<ffi::JSCValue, ffi::JSCValueClass>);

    match fn {
        type_ => || ffi::jsc_value_get_type(),
    }
}

impl Value {
    //#[doc(alias = "jsc_value_new_array")]
    //pub fn new_array<P: IsA<Context>>(context: &P, first_item_type: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Value {
    //    unsafe { TODO: call ffi:jsc_value_new_array() }
    //}

    #[doc(alias = "jsc_value_new_array_from_garray")]
    pub fn new_array_from_garray<P: IsA<Context>>(context: &P, array: &[Value]) -> Value {
        unsafe {
            from_glib_full(ffi::jsc_value_new_array_from_garray(context.as_ref().to_glib_none().0, array.to_glib_none().0))
        }
    }

    #[doc(alias = "jsc_value_new_array_from_strv")]
    pub fn new_array_from_strv<P: IsA<Context>>(context: &P, strv: &[&str]) -> Value {
        unsafe {
            from_glib_full(ffi::jsc_value_new_array_from_strv(context.as_ref().to_glib_none().0, strv.to_glib_none().0))
        }
    }

    #[doc(alias = "jsc_value_new_boolean")]
    pub fn new_boolean<P: IsA<Context>>(context: &P, value: bool) -> Value {
        unsafe {
            from_glib_full(ffi::jsc_value_new_boolean(context.as_ref().to_glib_none().0, value.into_glib()))
        }
    }

    #[doc(alias = "jsc_value_new_from_json")]
    #[doc(alias = "new_from_json")]
    pub fn from_json<P: IsA<Context>>(context: &P, json: &str) -> Value {
        unsafe {
            from_glib_full(ffi::jsc_value_new_from_json(context.as_ref().to_glib_none().0, json.to_glib_none().0))
        }
    }

    //#[doc(alias = "jsc_value_new_function")]
    //pub fn new_function<P: IsA<Context>, Q: Fn() + 'static>(context: &P, name: Option<&str>, callback: Q, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, return_type: glib::types::Type, n_params: u32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Value {
    //    unsafe { TODO: call ffi:jsc_value_new_function() }
    //}

    //#[doc(alias = "jsc_value_new_function_variadic")]
    //pub fn new_function_variadic<P: IsA<Context>>(context: &P, name: Option<&str>, callback: /*Unimplemented*/Fn(/*Ignored*/glib::PtrArray) -> Value, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, return_type: glib::types::Type) -> Value {
    //    unsafe { TODO: call ffi:jsc_value_new_function_variadic() }
    //}

    //#[doc(alias = "jsc_value_new_functionv")]
    //pub fn new_functionv<P: IsA<Context>, Q: Fn() + 'static>(context: &P, name: Option<&str>, callback: Q, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, return_type: glib::types::Type, n_parameters: u32) -> Value {
    //    unsafe { TODO: call ffi:jsc_value_new_functionv() }
    //}

    #[doc(alias = "jsc_value_new_null")]
    pub fn new_null<P: IsA<Context>>(context: &P) -> Value {
        unsafe {
            from_glib_full(ffi::jsc_value_new_null(context.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "jsc_value_new_number")]
    pub fn new_number<P: IsA<Context>>(context: &P, number: f64) -> Value {
        unsafe {
            from_glib_full(ffi::jsc_value_new_number(context.as_ref().to_glib_none().0, number))
        }
    }

    //#[doc(alias = "jsc_value_new_object")]
    //pub fn new_object<P: IsA<Context>>(context: &P, instance: /*Unimplemented*/Option<Fundamental: Pointer>, jsc_class: Option<&Class>) -> Value {
    //    unsafe { TODO: call ffi:jsc_value_new_object() }
    //}

    #[doc(alias = "jsc_value_new_string")]
    pub fn new_string<P: IsA<Context>>(context: &P, string: Option<&str>) -> Value {
        unsafe {
            from_glib_full(ffi::jsc_value_new_string(context.as_ref().to_glib_none().0, string.to_glib_none().0))
        }
    }

    #[doc(alias = "jsc_value_new_string_from_bytes")]
    pub fn new_string_from_bytes<P: IsA<Context>>(context: &P, bytes: Option<&glib::Bytes>) -> Value {
        unsafe {
            from_glib_full(ffi::jsc_value_new_string_from_bytes(context.as_ref().to_glib_none().0, bytes.to_glib_none().0))
        }
    }

    #[doc(alias = "jsc_value_new_undefined")]
    pub fn new_undefined<P: IsA<Context>>(context: &P) -> Value {
        unsafe {
            from_glib_full(ffi::jsc_value_new_undefined(context.as_ref().to_glib_none().0))
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Value`] objects.
            ///
            /// This method returns an instance of [`ValueBuilder`] which can be used to create [`Value`] objects.
            pub fn builder() -> ValueBuilder {
                ValueBuilder::default()
            }
        
}

impl fmt::Display for Value {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&ValueExt::to_str(self))
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Value`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct ValueBuilder {
    context: Option<Context>,
}

impl ValueBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ValueBuilder`].
    pub fn new() -> Self {
        Self::default()
    }


    // rustdoc-stripper-ignore-next
    /// Build the [`Value`].
    pub fn build(self) -> Value {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
if let Some(ref context) = self.context {
                properties.push(("context", context));
            }
        glib::Object::new::<Value>(&properties)
                .expect("Failed to create an instance of Value")

    }

    pub fn context<P: IsA<Context>>(mut self, context: &P) -> Self {
        self.context = Some(context.clone().upcast());
        self
    }
}

pub const NONE_VALUE: Option<&Value> = None;

pub trait ValueExt: 'static {
    //#[doc(alias = "jsc_value_constructor_call")]
    //fn constructor_call(&self, first_parameter_type: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Value>;

    #[doc(alias = "jsc_value_constructor_callv")]
    fn constructor_callv(&self, parameters: &[Value]) -> Option<Value>;

    //#[doc(alias = "jsc_value_function_call")]
    //fn function_call(&self, first_parameter_type: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Value>;

    #[doc(alias = "jsc_value_function_callv")]
    fn function_callv(&self, parameters: &[Value]) -> Option<Value>;

    #[doc(alias = "jsc_value_get_context")]
    #[doc(alias = "get_context")]
    fn context(&self) -> Option<Context>;

    #[doc(alias = "jsc_value_is_array")]
    fn is_array(&self) -> bool;

    #[doc(alias = "jsc_value_is_boolean")]
    fn is_boolean(&self) -> bool;

    #[doc(alias = "jsc_value_is_constructor")]
    fn is_constructor(&self) -> bool;

    #[doc(alias = "jsc_value_is_function")]
    fn is_function(&self) -> bool;

    #[doc(alias = "jsc_value_is_null")]
    fn is_null(&self) -> bool;

    #[doc(alias = "jsc_value_is_number")]
    fn is_number(&self) -> bool;

    #[doc(alias = "jsc_value_is_object")]
    fn is_object(&self) -> bool;

    #[doc(alias = "jsc_value_is_string")]
    fn is_string(&self) -> bool;

    #[doc(alias = "jsc_value_is_undefined")]
    fn is_undefined(&self) -> bool;

    #[doc(alias = "jsc_value_object_define_property_accessor")]
    fn object_define_property_accessor(&self, property_name: &str, flags: ValuePropertyFlags, property_type: glib::types::Type, getter: Option<Box_<dyn Fn() -> Value + 'static>>, setter: Option<Box_<dyn Fn(&Value) + 'static>>);

    #[doc(alias = "jsc_value_object_define_property_data")]
    fn object_define_property_data<P: IsA<Value>>(&self, property_name: &str, flags: ValuePropertyFlags, property_value: Option<&P>);

    #[doc(alias = "jsc_value_object_delete_property")]
    fn object_delete_property(&self, name: &str) -> bool;

    #[doc(alias = "jsc_value_object_enumerate_properties")]
    fn object_enumerate_properties(&self) -> Vec<glib::GString>;

    #[doc(alias = "jsc_value_object_get_property")]
    fn object_get_property(&self, name: &str) -> Option<Value>;

    #[doc(alias = "jsc_value_object_get_property_at_index")]
    fn object_get_property_at_index(&self, index: u32) -> Option<Value>;

    #[doc(alias = "jsc_value_object_has_property")]
    fn object_has_property(&self, name: &str) -> bool;

    //#[doc(alias = "jsc_value_object_invoke_method")]
    //fn object_invoke_method(&self, name: &str, first_parameter_type: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Value>;

    #[doc(alias = "jsc_value_object_invoke_methodv")]
    fn object_invoke_methodv(&self, name: &str, parameters: &[Value]) -> Option<Value>;

    #[doc(alias = "jsc_value_object_is_instance_of")]
    fn object_is_instance_of(&self, name: &str) -> bool;

    #[doc(alias = "jsc_value_object_set_property")]
    fn object_set_property<P: IsA<Value>>(&self, name: &str, property: &P);

    #[doc(alias = "jsc_value_object_set_property_at_index")]
    fn object_set_property_at_index<P: IsA<Value>>(&self, index: u32, property: &P);

    #[doc(alias = "jsc_value_to_boolean")]
    fn to_boolean(&self) -> bool;

    #[doc(alias = "jsc_value_to_double")]
    fn to_double(&self) -> f64;

    #[doc(alias = "jsc_value_to_int32")]
    fn to_int32(&self) -> i32;

    #[doc(alias = "jsc_value_to_json")]
    fn to_json(&self, indent: u32) -> Option<glib::GString>;

    #[doc(alias = "jsc_value_to_string")]
    #[doc(alias = "to_string")]
    fn to_str(&self) -> glib::GString;

    #[doc(alias = "jsc_value_to_string_as_bytes")]
    fn to_string_as_bytes(&self) -> Option<glib::Bytes>;
}

impl<O: IsA<Value>> ValueExt for O {
    //fn constructor_call(&self, first_parameter_type: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Value> {
    //    unsafe { TODO: call ffi:jsc_value_constructor_call() }
    //}

    fn constructor_callv(&self, parameters: &[Value]) -> Option<Value> {
        let n_parameters = parameters.len() as u32;
        unsafe {
            from_glib_full(ffi::jsc_value_constructor_callv(self.as_ref().to_glib_none().0, n_parameters, parameters.to_glib_none().0))
        }
    }

    //fn function_call(&self, first_parameter_type: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Value> {
    //    unsafe { TODO: call ffi:jsc_value_function_call() }
    //}

    fn function_callv(&self, parameters: &[Value]) -> Option<Value> {
        let n_parameters = parameters.len() as u32;
        unsafe {
            from_glib_full(ffi::jsc_value_function_callv(self.as_ref().to_glib_none().0, n_parameters, parameters.to_glib_none().0))
        }
    }

    fn context(&self) -> Option<Context> {
        unsafe {
            from_glib_none(ffi::jsc_value_get_context(self.as_ref().to_glib_none().0))
        }
    }

    fn is_array(&self) -> bool {
        unsafe {
            from_glib(ffi::jsc_value_is_array(self.as_ref().to_glib_none().0))
        }
    }

    fn is_boolean(&self) -> bool {
        unsafe {
            from_glib(ffi::jsc_value_is_boolean(self.as_ref().to_glib_none().0))
        }
    }

    fn is_constructor(&self) -> bool {
        unsafe {
            from_glib(ffi::jsc_value_is_constructor(self.as_ref().to_glib_none().0))
        }
    }

    fn is_function(&self) -> bool {
        unsafe {
            from_glib(ffi::jsc_value_is_function(self.as_ref().to_glib_none().0))
        }
    }

    fn is_null(&self) -> bool {
        unsafe {
            from_glib(ffi::jsc_value_is_null(self.as_ref().to_glib_none().0))
        }
    }

    fn is_number(&self) -> bool {
        unsafe {
            from_glib(ffi::jsc_value_is_number(self.as_ref().to_glib_none().0))
        }
    }

    fn is_object(&self) -> bool {
        unsafe {
            from_glib(ffi::jsc_value_is_object(self.as_ref().to_glib_none().0))
        }
    }

    fn is_string(&self) -> bool {
        unsafe {
            from_glib(ffi::jsc_value_is_string(self.as_ref().to_glib_none().0))
        }
    }

    fn is_undefined(&self) -> bool {
        unsafe {
            from_glib(ffi::jsc_value_is_undefined(self.as_ref().to_glib_none().0))
        }
    }

    fn object_define_property_accessor(&self, property_name: &str, flags: ValuePropertyFlags, property_type: glib::types::Type, getter: Option<Box_<dyn Fn() -> Value + 'static>>, setter: Option<Box_<dyn Fn(&Value) + 'static>>) {
        let getter_data: Option<Box_<dyn Fn() -> Value + 'static>> = getter;
        unsafe extern "C" fn getter_func(user_data: glib::ffi::gpointer) -> *mut ffi::JSCValue {
            let callback: &Box_<(&Option<Box_<dyn Fn() -> Value + 'static>>, &Option<Box_<dyn Fn(&Value) + 'static>>)> = &*(user_data as *mut _);
            let res = if let Some(ref callback) = callback.0 {
                callback()
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib_full()
        }
        let getter = if getter_data.is_some() { Some(getter_func as _) } else { None };
        let setter_data: Option<Box_<dyn Fn(&Value) + 'static>> = setter;
        unsafe extern "C" fn setter_func(value: *mut ffi::JSCValue, user_data: glib::ffi::gpointer) {
            let value = from_glib_borrow(value);
            let callback: &Box_<(&Option<Box_<dyn Fn() -> Value + 'static>>, &Option<Box_<dyn Fn(&Value) + 'static>>)> = &*(user_data as *mut _);
            if let Some(ref callback) = callback.1 {
                callback(&value)
            } else {
                panic!("cannot get closure...")
            };
        }
        let setter = if setter_data.is_some() { Some(setter_func as _) } else { None };
        unsafe extern "C" fn destroy_notify_func(data: glib::ffi::gpointer) {
            let _callback: Box_<(&Option<Box_<dyn Fn() -> Value + 'static>>, &Option<Box_<dyn Fn(&Value) + 'static>>)> = Box_::from_raw(data as *mut _);
        }
        let destroy_call7 = Some(destroy_notify_func as _);
        let super_callback0: Box_<(&Option<Box_<dyn Fn() -> Value + 'static>>, &Option<Box_<dyn Fn(&Value) + 'static>>)> = Box_::new((&getter_data, &setter_data));
        unsafe {
            ffi::jsc_value_object_define_property_accessor(self.as_ref().to_glib_none().0, property_name.to_glib_none().0, flags.into_glib(), property_type.into_glib(), getter, setter, Box_::into_raw(super_callback0) as *mut _, destroy_call7);
        }
    }

    fn object_define_property_data<P: IsA<Value>>(&self, property_name: &str, flags: ValuePropertyFlags, property_value: Option<&P>) {
        unsafe {
            ffi::jsc_value_object_define_property_data(self.as_ref().to_glib_none().0, property_name.to_glib_none().0, flags.into_glib(), property_value.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn object_delete_property(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::jsc_value_object_delete_property(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn object_enumerate_properties(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::jsc_value_object_enumerate_properties(self.as_ref().to_glib_none().0))
        }
    }

    fn object_get_property(&self, name: &str) -> Option<Value> {
        unsafe {
            from_glib_full(ffi::jsc_value_object_get_property(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn object_get_property_at_index(&self, index: u32) -> Option<Value> {
        unsafe {
            from_glib_full(ffi::jsc_value_object_get_property_at_index(self.as_ref().to_glib_none().0, index))
        }
    }

    fn object_has_property(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::jsc_value_object_has_property(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    //fn object_invoke_method(&self, name: &str, first_parameter_type: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Value> {
    //    unsafe { TODO: call ffi:jsc_value_object_invoke_method() }
    //}

    fn object_invoke_methodv(&self, name: &str, parameters: &[Value]) -> Option<Value> {
        let n_parameters = parameters.len() as u32;
        unsafe {
            from_glib_full(ffi::jsc_value_object_invoke_methodv(self.as_ref().to_glib_none().0, name.to_glib_none().0, n_parameters, parameters.to_glib_none().0))
        }
    }

    fn object_is_instance_of(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::jsc_value_object_is_instance_of(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn object_set_property<P: IsA<Value>>(&self, name: &str, property: &P) {
        unsafe {
            ffi::jsc_value_object_set_property(self.as_ref().to_glib_none().0, name.to_glib_none().0, property.as_ref().to_glib_none().0);
        }
    }

    fn object_set_property_at_index<P: IsA<Value>>(&self, index: u32, property: &P) {
        unsafe {
            ffi::jsc_value_object_set_property_at_index(self.as_ref().to_glib_none().0, index, property.as_ref().to_glib_none().0);
        }
    }

    fn to_boolean(&self) -> bool {
        unsafe {
            from_glib(ffi::jsc_value_to_boolean(self.as_ref().to_glib_none().0))
        }
    }

    fn to_double(&self) -> f64 {
        unsafe {
            ffi::jsc_value_to_double(self.as_ref().to_glib_none().0)
        }
    }

    fn to_int32(&self) -> i32 {
        unsafe {
            ffi::jsc_value_to_int32(self.as_ref().to_glib_none().0)
        }
    }

    fn to_json(&self, indent: u32) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::jsc_value_to_json(self.as_ref().to_glib_none().0, indent))
        }
    }

    fn to_str(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::jsc_value_to_string(self.as_ref().to_glib_none().0))
        }
    }

    fn to_string_as_bytes(&self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_full(ffi::jsc_value_to_string_as_bytes(self.as_ref().to_glib_none().0))
        }
    }
}
