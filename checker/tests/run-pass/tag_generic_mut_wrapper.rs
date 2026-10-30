// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use mirai_annotations::*;

struct InputTagKind<const MASK: TagPropagationSet>;

type InputTag = InputTagKind<TAG_PROPAGATION_ALL>;

struct Builder {
    base_constraint: u8,
    extension_constraint: u8,
}

trait ConstraintBuilder {
    fn assert_extension_zero<I: Into<u8>>(&mut self, value: I);
}

impl ConstraintBuilder for Builder {
    fn assert_extension_zero<I: Into<u8>>(&mut self, value: I) {
        self.extension_constraint |= value.into();
    }
}

struct FilteredBuilder<'a, B> {
    inner: &'a mut B,
}

impl<B: ConstraintBuilder> ConstraintBuilder for FilteredBuilder<'_, B> {
    fn assert_extension_zero<I: Into<u8>>(&mut self, value: I) {
        self.inner.assert_extension_zero(value);
    }
}

fn constrain<B: ConstraintBuilder>(builder: &mut B, value: u8) {
    let mut filtered = FilteredBuilder { inner: builder };
    filtered.assert_extension_zero(value);
}

pub fn main() {
    let mut builder = Builder {
        base_constraint: 0,
        extension_constraint: 0,
    };
    let extension_value = 0;
    add_tag!(&builder.base_constraint, InputTag);
    add_tag!(&extension_value, InputTag);

    constrain(&mut builder, 0);
    builder.extension_constraint = 0;
    constrain(&mut builder, extension_value);
    constrain(&mut builder, 0);

    verify!(has_tag!(&builder.base_constraint, InputTag));
    verify!(has_tag!(&builder.extension_constraint, InputTag));
}
