use std;

use jni_sys::*;

#[derive(Clone)]
pub struct Value<'a> {
  handle: jvalue,
  _data: std::marker::PhantomData<&'a jvalue>
}

impl<'a> Value<'a> {
  pub(crate) fn from_handle(handle: jvalue) -> Value<'a> {
    return Value { handle: handle, _data: std::marker::PhantomData };
  }

  pub(crate) fn as_handle(&self) -> jvalue {
    return self.handle;
  }

  pub fn from_boolean(value: bool) -> Value<'a> {
    let mut handle = jvalue::default();

    unsafe { *handle.z() = if value { 1 } else { 0 } };

    return Value::from_handle(handle);
  }
  
  pub fn from_i8(value: i8) -> Value<'a> {
    let mut handle = jvalue::default();

    unsafe { *handle.b() = value };

    return Value::from_handle(handle);
  }
  
  pub fn from_i16(value: i16) -> Value<'a> {
    let mut handle = jvalue::default();

    unsafe { *handle.s() = value };

    return Value::from_handle(handle);
  }
  
  pub fn from_i32(value: i32) -> Value<'a> {
    let mut handle = jvalue::default();

    unsafe { *handle.i() = value };

    return Value::from_handle(handle);
  }
  
  pub fn from_i64(value: i64) -> Value<'a> {
    let mut handle = jvalue::default();

    unsafe { *handle.j() = value };

    return Value::from_handle(handle);
  }
  
  pub fn from_object(value: &::Object) -> Value<'a> {
    let mut handle = jvalue::default();

    unsafe { *handle.l() = value.as_handle() };

    return Value::from_handle(handle);
  }
  
  pub fn from_string(value: &::String) -> Value<'a> {
    let mut handle = jvalue::default();

    unsafe { *handle.l() = value.as_handle() };

    return Value::from_handle(handle);
  }
}
