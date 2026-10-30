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

#[repr(transparent)]
struct Cell(u8);

#[repr(C)]
struct Columns {
    prefix: [Cell; 1],
    last: Cell,
}

pub fn main() {
    let row = [Cell(0), Cell(0)];
    add_tag!(&row[1].0, InputTag);
    let (_, columns, _) = unsafe { row.as_slice().align_to::<Columns>() };
    let sink = columns[0].last.0;
    verify!(has_tag!(&sink, InputTag));
}
