// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

fn range_index(values: &[u8; 8]) {
    for index in 0..8 {
        let _ = values[index];
    }
}

fn helper_index(values: &[u8; 64]) {
    for index in 0..8 {
        let offset = offset(index, 0);
        let _ = values[offset + 2];
    }
}

const fn offset(index: usize, field: usize) -> usize {
    const OFFSETS: [usize; 8] = [22, 23, 24, 25, 26, 17, 28, 29];

    if index >= OFFSETS.len() {
        panic!("index out of range");
    }
    OFFSETS[index] * 2 + field
}

pub fn main() {
    range_index(&[0; 8]);
    helper_index(&[0; 64]);
}
