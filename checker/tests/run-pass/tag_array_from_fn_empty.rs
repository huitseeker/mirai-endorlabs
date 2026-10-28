// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use mirai_annotations::*;

pub fn main() {
    let mut called = false;
    let output: [u8; 0] = core::array::from_fn(|_| {
        called = true;
        1
    });
    verify!(!called);
    verify!(output.len() == 0);
}
