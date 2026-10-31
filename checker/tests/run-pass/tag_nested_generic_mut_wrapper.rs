// Copyright (c) Facebook Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use mirai_annotations::*;

struct InputTagKind<const MASK: TagPropagationSet>;

const INPUT_MASK: TagPropagationSet = TAG_PROPAGATION_ALL
    & !TagPropagation::SubComponent.into_set()
    & !TagPropagation::SuperComponent.into_set();

type InputTag = InputTagKind<INPUT_MASK>;

trait LookupBuilder {
    type Expr: Copy;
    type Column<'a>
    where
        Self: 'a;

    fn periodic_values(&self) -> &[Self::Expr];
    fn next_column<'a, R>(&'a mut self, f: impl FnOnce(&mut Self::Column<'a>) -> R) -> R;
}

struct Builder<'a> {
    input: u8,
    periodic_values: &'a [u16],
    saw_input_tag: bool,
}

impl LookupBuilder for Builder<'_> {
    type Expr = u16;
    type Column<'a>
        = ()
    where
        Self: 'a;

    fn periodic_values(&self) -> &[Self::Expr] {
        self.periodic_values
    }

    fn next_column<'a, R>(&'a mut self, f: impl FnOnce(&mut Self::Column<'a>) -> R) -> R {
        let result = f(&mut ());
        if has_tag!(&self.input, InputTag) {
            self.saw_input_tag = true;
        }
        result
    }
}

fn emit_column<B: LookupBuilder>(builder: &mut B) {
    builder.next_column(|_| {});
}

fn selectors<T: Copy>(periodic_values: &[T]) -> [T; 14] {
    assume!(periodic_values.len() >= 14);
    core::array::from_fn(|index| periodic_values[index])
}

fn evaluate<B: LookupBuilder>(builder: &mut B) {
    let periodic_values: Vec<B::Expr> =
        builder.periodic_values().iter().map(|value| *value).collect();
    let _selectors = selectors(&periodic_values);
    emit_column(builder);
}

pub fn main() {
    let input = 0;
    let periodic_values = [0; 14];
    add_tag!(&input, InputTag);
    let mut builder = Builder {
        input,
        periodic_values: &periodic_values,
        saw_input_tag: false,
    };

    evaluate(&mut builder);

    verify!(builder.saw_input_tag);
}
