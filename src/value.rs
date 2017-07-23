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
    return Value::from_handle(jvalue { z: if value { 1 } else { 0 } });
  }

  pub fn from_i8(value: i8) -> Value<'a> {
    return Value::from_handle(jvalue { b: value });
  }

  pub fn from_i16(value: i16) -> Value<'a> {
    return Value::from_handle(jvalue { s: value });
  }

  pub fn from_i32(value: i32) -> Value<'a> {
    return Value::from_handle(jvalue { i: value });
  }

  pub fn from_i64(value: i64) -> Value<'a> {
    return Value::from_handle(jvalue { j: value });
  }

  pub fn from_object(value: &::Object) -> Value<'a> {
    return Value::from_handle(jvalue { l: value.as_handle() });
  }

  pub fn from_string(value: &::String) -> Value<'a> {
    return Value::from_handle(jvalue { l: value.as_handle() });
  }
}
