use jni_sys::*;

#[derive(Clone, Debug)]
pub struct Method(jmethodID);

impl Method {
  pub(crate) fn from_handle(handle: jmethodID) -> Method {
    return Method(handle);
  }

  pub(crate) fn as_handle(&self) -> jmethodID {
    return self.0;
  }
}
