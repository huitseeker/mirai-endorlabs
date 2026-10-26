// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::ffi::CStr;

pub fn c_string_constant() -> &'static CStr {
    c"mirai"
}

pub fn main() {
    let _ = c_string_constant().to_bytes_with_nul();
}
