// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::ops::{Add, AddAssign, Mul, Sub};

use mirai_annotations::*;

struct AuxiliaryTagKind<const MASK: TagPropagationSet>;
struct PermutationValueTagKind<const MASK: TagPropagationSet>;
struct ColumnTagKind<const MASK: TagPropagationSet>;

const INPUT_MASK: TagPropagationSet = TAG_PROPAGATION_ALL
    & !TagPropagation::SubComponent.into_set()
    & !TagPropagation::SuperComponent.into_set();

type AuxiliaryTag = AuxiliaryTagKind<INPUT_MASK>;
type PermutationValueTag = PermutationValueTagKind<INPUT_MASK>;
type ColumnTag = ColumnTagKind<INPUT_MASK>;

#[derive(Clone, Copy)]
struct Ext(u8);

impl Add for Ext {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl AddAssign for Ext {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Ext {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Mul for Ext {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

trait ExtensionExpr:
    Copy + Add<Output = Self> + AddAssign + Sub<Output = Self> + Mul<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;
}

impl ExtensionExpr for Ext {
    const ZERO: Self = Self(0);
    const ONE: Self = Self(0);
}

#[derive(Clone, Copy)]
struct RowWindow<'a, T> {
    current: &'a [T],
    next: &'a [T],
}

impl<T> RowWindow<'_, T> {
    fn current_slice(&self) -> &[T] {
        self.current
    }

    fn next_slice(&self) -> &[T] {
        self.next
    }
}

trait ExtensionBuilder {
    type ExprEF: ExtensionExpr;
    type VarEF: Copy + Into<Self::ExprEF>;
    type PermutationWindow: Copy;

    fn permutation(&self) -> Self::PermutationWindow;
    fn permutation_values(&self) -> &[Self::VarEF];
    fn assert_zero_ext(&mut self, value: Self::ExprEF);

    fn when_first_row(&mut self) -> FilteredBuilder<'_, Self>
    where
        Self: Sized,
    {
        FilteredBuilder { inner: self }
    }
}

trait WindowAccess<T> {
    fn current_slice(&self) -> &[T];
    fn next_slice(&self) -> &[T];
}

impl<T> WindowAccess<T> for RowWindow<'_, T> {
    fn current_slice(&self) -> &[T] {
        self.current_slice()
    }

    fn next_slice(&self) -> &[T] {
        self.next_slice()
    }
}

struct FilteredBuilder<'a, B> {
    inner: &'a mut B,
}

impl<B: ExtensionBuilder> FilteredBuilder<'_, B> {
    fn assert_zero_ext(&mut self, value: B::ExprEF) {
        self.inner.assert_zero_ext(value);
    }
}

struct Builder<'a> {
    permutation: RowWindow<'a, Ext>,
    permutation_values: &'a [Ext],
    extension_constraint: u8,
}

impl<'a> ExtensionBuilder for Builder<'a> {
    type ExprEF = Ext;
    type VarEF = Ext;
    type PermutationWindow = RowWindow<'a, Ext>;

    fn permutation(&self) -> Self::PermutationWindow {
        self.permutation
    }

    fn permutation_values(&self) -> &[Self::VarEF] {
        self.permutation_values
    }

    fn assert_zero_ext(&mut self, value: Self::ExprEF) {
        self.extension_constraint |= value.0;
    }
}

fn next_column<B>(builder: &mut B, u: B::ExprEF, v: B::ExprEF, column_index: usize)
where
    B: ExtensionBuilder,
    B::PermutationWindow: WindowAccess<B::VarEF>,
{
    if column_index == 0 {
        let (acc, acc_next, sigma_prime) = {
            let permutation = builder.permutation();
            let acc = permutation.current_slice()[0].into();
            let acc_next = permutation.next_slice()[0].into();
            let sigma_prime = builder.permutation_values()[0].into();
            (acc, acc_next, sigma_prime)
        };

        let all_current_sum = {
            let permutation = builder.permutation();
            let current = permutation.current_slice();
            let mut sum = B::ExprEF::ZERO;
            for &auxiliary in current {
                sum += auxiliary.into();
            }
            sum
        };

        builder.when_first_row().assert_zero_ext(acc);
        builder.assert_zero_ext(u * (acc_next - all_current_sum + sigma_prime) - v);
    } else {
        builder.assert_zero_ext(B::ExprEF::ZERO);
    }
}

fn emit<B>(builder: &mut B, column_value: B::ExprEF)
where
    B: ExtensionBuilder,
    B::PermutationWindow: WindowAccess<B::VarEF>,
{
    next_column(
        builder,
        column_value,
        B::ExprEF::ONE,
        abstract_value!(0usize),
    );
}

fn complex_column(mut left: Ext, mut right: Ext) -> Ext {
    macro_rules! grow {
        () => {{
            let next = left * right + left;
            left = right;
            right = next;
        }};
    }

    grow!();
    grow!();
    grow!();
    grow!();
    grow!();
    grow!();
    grow!();
    grow!();
    grow!();
    grow!();
    grow!();
    grow!();
    grow!();
    grow!();
    grow!();
    grow!();
    grow!();
    grow!();
    grow!();
    right = left * right + left;
    right
}

pub fn main() {
    let current = [Ext(0), Ext(0), Ext(0)];
    let next = [Ext(0), Ext(0), Ext(0)];
    let permutation_values = [Ext(0)];
    let column_value = Ext(abstract_value!(0u8));
    add_tag!(&current[0].0, AuxiliaryTag);
    add_tag!(&permutation_values[0].0, PermutationValueTag);
    add_tag!(&column_value.0, ColumnTag);

    let mut builder = Builder {
        permutation: RowWindow {
            current: &current,
            next: &next,
        },
        permutation_values: &permutation_values,
        extension_constraint: 0,
    };
    let complex_column_value = complex_column(column_value, Ext(abstract_value!(1u8)));
    emit(&mut builder, complex_column_value);
    emit(&mut builder, Ext(0));
    emit(&mut builder, Ext(0));
    emit(&mut builder, Ext(0));

    verify!(has_tag!(&builder.extension_constraint, AuxiliaryTag));
    verify!(has_tag!(
        &builder.extension_constraint,
        PermutationValueTag
    ));
    verify!(has_tag!(&builder.extension_constraint, ColumnTag));
}
