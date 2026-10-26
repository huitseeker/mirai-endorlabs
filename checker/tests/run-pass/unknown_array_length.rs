// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// Regression test for an array length that cannot be reduced by
// `Const::try_to_target_usize` while MIRAI deserializes a constant.

const ARRAY_LEN: usize = 2;

#[derive(Copy, Clone)]
struct Holder {
    bytes: [u8; ARRAY_LEN],
}

const HOLDER: Holder = Holder {
    bytes: [7; ARRAY_LEN],
};

pub fn first_byte() -> u8 {
    HOLDER.bytes[0]
}
