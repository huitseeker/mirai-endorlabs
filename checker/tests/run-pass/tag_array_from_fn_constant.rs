// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use mirai_annotations::*;

struct InputTagKind<const MASK: TagPropagationSet>;

const INPUT_MASK: TagPropagationSet = TAG_PROPAGATION_ALL
    & !TagPropagation::SubComponent.into_set()
    & !TagPropagation::SuperComponent.into_set();

type InputTag = InputTagKind<INPUT_MASK>;

#[derive(Clone, Copy)]
struct Cell(u8);

pub fn main() {
    let input = [Cell(0), Cell(1)];
    add_tag!(&input[0].0, InputTag);
    let output: [Cell; 2] = core::array::from_fn(|_| input[0]);
    verify!(has_tag!(&output[1].0, InputTag));
}
