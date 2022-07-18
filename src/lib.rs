/*
* Copyright (c) 2022 XXIV
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/
use std::os::raw::c_char;
use std::ffi::CString;
use std::ffi::CStr;
use std::slice;
use urlencoding::encode;
use urlencoding::encode_binary;
use urlencoding::decode;
use urlencoding::decode_binary;

/// Percent-encodes every byte except alphanumerics and -, _, ., ~. Assumes UTF-8 encoding.
/// 
/// Example:
/// * *
/// int main()
/// {
///     char* res = url_encoding_encode("This string will be URL encoded.");
///     printf("%s\n", res);
///     url_encoding_free(res);
///     return 0;
/// }
/// * *
/// 
/// @param data
/// @return dynamic string
#[no_mangle]
unsafe extern "C" fn url_encoding_encode(data: *const c_char) -> *mut c_char {
  if data.is_null() {
    return std::ptr::null_mut();
  }
  let str = match CStr::from_ptr(data).to_str() {
    Ok(s) => s,
    Err(_) => return std::ptr::null_mut()
  };
  let res = encode(str);
  match CString::new(res.into_owned()) {
    Ok(s) => s.into_raw(),
    Err(_) => std::ptr::null_mut()
  }
}

/// Percent-encodes every byte except alphanumerics and -, _, ., ~.
/// 
/// Example:
/// * *
/// int main()
/// {
///     const unsigned char* str = "This string will be URL encoded.";
///     char* res = url_encoding_encode_binary(str, strlen(str));
///     printf("%s\n", res);
///     url_encoding_free(res);
///     return 0;
/// }
/// * *
/// 
/// @param data
/// @param length data length
/// @return dynamic string
#[no_mangle]
unsafe extern "C" fn url_encoding_encode_binary(data: *const u8, length: usize) -> *mut c_char {
  if data.is_null() {
    return std::ptr::null_mut();
  }
  let array = slice::from_raw_parts(data, length);
  let res = encode_binary(array);
  match CString::new(res.into_owned()) {
    Ok(s) => s.into_raw(),
    Err(_) => std::ptr::null_mut()
  }
}

/// Decode percent-encoded string assuming UTF-8 encoding.
/// 
/// Example:
/// * *
/// int main()
/// {
///     char* res = url_encoding_decode("%F0%9F%91%BE%20Exterminate%21");
///     printf("%s\n", res);
///     url_encoding_free(res);
///     return 0;
/// }
/// * *
/// 
/// @param data
/// @return dynamic string
#[no_mangle]
unsafe extern "C" fn url_encoding_decode(data: *const c_char) -> *mut c_char {
  if data.is_null() {
    return std::ptr::null_mut();
  }
  let str = match CStr::from_ptr(data).to_str() {
    Ok(s) => s,
    Err(_) => return std::ptr::null_mut()
  };
  let res = match decode(str) {
    Ok(s) => s,
    Err(_) => return std::ptr::null_mut()
  };
  match CString::new(res.into_owned()) {
    Ok(s) => s.into_raw(),
    Err(_) => std::ptr::null_mut()
  }
}

/// Decode percent-encoded string as binary data, in any encoding.
/// 
/// Example:
/// * *
/// int main()
/// {
///     const unsigned char* str = "%F1%F2%F3%C0%C1%C2";
///     char* res = url_encoding_decode_binary(str, strlen(str));
///     printf("%s\n", res);
///     url_encoding_free(res);
///     return 0;
/// }
/// * *
/// 
/// @param data
/// @param length data length
/// @return dynamic string
#[no_mangle]
unsafe extern "C" fn url_encoding_decode_binary(data: *const u8, length: usize) -> *mut c_char {
  if data.is_null() {
    return std::ptr::null_mut();
  }
  let array = slice::from_raw_parts(data, length);
  let res = decode_binary(array);
  match CString::new(res.into_owned()) {
    Ok(s) => s.into_raw(),
    Err(_) => std::ptr::null_mut()
  }
}

/// function to free the memory after using urlencoding functions
///
/// @param ptr string returned from urlencoding functions
#[no_mangle]
unsafe extern "C" fn url_encoding_free(ptr: *mut c_char) {
  if !ptr.is_null() {
    _ = CString::from_raw(ptr);
  }
}