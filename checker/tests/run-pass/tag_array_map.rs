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

fn generic_map<I: Copy, const N: usize>(input: [I; N]) -> [I; N] {
    input.map(|cell| cell)
}

pub fn main() {
    let input = [Cell(0), Cell(1)];
    add_tag!(&input[0].0, InputTag);

    let output = input.map(|cell| Cell(cell.0 + 1));
    verify!(has_tag!(&output[0].0, InputTag));
    verify!(!has_tag!(&output[1].0, InputTag));

    let generic_output = generic_map(input);
    verify!(has_tag!(&generic_output[0].0, InputTag));
    verify!(!has_tag!(&generic_output[1].0, InputTag));

    let mut selected = input[0];
    let stateful = input.map(|_| {
        let result = selected;
        selected = input[1];
        result
    });
    verify!(has_tag!(&stateful[0].0, InputTag));
    verify!(!has_tag!(&stateful[1].0, InputTag));
}
