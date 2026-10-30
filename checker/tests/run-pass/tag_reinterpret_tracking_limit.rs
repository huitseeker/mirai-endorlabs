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

#[repr(C)]
struct BelowLimit([Cell; 99]);

#[repr(C)]
struct AtLimit([Cell; 100]);

fn borrow_below_limit(values: &[Cell]) -> &BelowLimit {
    let (_, layouts, _) = unsafe { values.align_to::<BelowLimit>() };
    &layouts[0]
}

fn borrow_at_limit(values: &[Cell]) -> &AtLimit {
    let (_, layouts, _) = unsafe { values.align_to::<AtLimit>() };
    &layouts[0]
}

pub fn main() {
    let below = [Cell(0); 99];
    add_tag!(&below[98].0, InputTag);
    let layout = borrow_below_limit(&below);
    verify!(has_tag!(&layout.0[98].0, InputTag));

    let at = [Cell(0); 100];
    let _ = borrow_at_limit(&at);
}
