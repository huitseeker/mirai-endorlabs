// Copyright (c) Facebook Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use mirai_annotations::verify;

pub fn main() {
    let source = [0_u8; 14];
    let values: Vec<u16> = source.iter().map(|value| (*value).into()).collect();

    verify!(values.len() == source.len());
    let _selectors: [u16; 14] = core::array::from_fn(|index| values[index]);
    verify!(values.len() == source.len());
}
