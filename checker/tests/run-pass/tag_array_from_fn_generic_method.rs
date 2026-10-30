// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use core::borrow::Borrow;

use mirai_annotations::*;

struct InputTagKind<const MASK: TagPropagationSet>;

const INPUT_MASK: TagPropagationSet = TAG_PROPAGATION_ALL
    & !TagPropagation::SubComponent.into_set()
    & !TagPropagation::SuperComponent.into_set();

type InputTag = InputTagKind<INPUT_MASK>;

#[derive(Clone, Copy)]
struct Cell(u8);

#[derive(Clone, Copy)]
#[repr(C)]
struct Cells<T> {
    values: [T; 2],
}

#[repr(C)]
struct PaddedCells<T> {
    values: [T; 4],
}

#[repr(C)]
struct Row<T> {
    values: [T; 4],
    clock: T,
}

impl<T: Copy> Cells<T> {
    fn copied(&self) -> [T; 2] {
        core::array::from_fn(|index| self.values[index])
    }
}

impl<T> PaddedCells<T> {
    fn middle(&self) -> &Cells<T> {
        self.values[1..3].borrow()
    }
}

impl<T> Row<T> {
    fn middle(&self) -> &Cells<T> {
        self.values[1..3].borrow()
    }
}

fn borrow_cells<T>(values: &[T]) -> &Cells<T> {
    let (_, cells, _) = unsafe { values.align_to::<Cells<T>>() };
    &cells[0]
}

fn borrow_row<T>(values: &[T]) -> &Row<T> {
    let (_, rows, _) = unsafe { values.align_to::<Row<T>>() };
    &rows[0]
}

impl<T> Borrow<Cells<T>> for [T] {
    fn borrow(&self) -> &Cells<T> {
        borrow_cells(self)
    }
}

#[derive(Clone, Copy)]
struct RowWindow<'a, T> {
    current: &'a [T],
    next: &'a [T],
}

impl<'a, T> RowWindow<'a, T> {
    fn current_slice(&self) -> &[T] {
        self.current
    }
}

struct Builder<'a> {
    main: RowWindow<'a, Cell>,
    constraint: u8,
}

impl<'a> Builder<'a> {
    fn main(&self) -> RowWindow<'a, Cell> {
        self.main
    }
}

trait ConstraintBuilder {
    type Var: Copy + Into<Cell>;

    fn assert_zero(&mut self, value: Cell);

    fn when(&mut self, condition: Cell) -> FilteredBuilder<'_, Self>
    where
        Self: Sized,
    {
        FilteredBuilder {
            inner: self,
            condition,
        }
    }

    fn assert_eq(&mut self, left: Cell, right: Cell) {
        self.assert_zero(Cell(left.0 ^ right.0));
    }

    fn assert_bool(&mut self, value: Cell) {
        self.assert_zero(Cell(value.0.wrapping_mul(value.0.wrapping_sub(1))));
    }
}

impl ConstraintBuilder for Builder<'_> {
    type Var = Cell;

    fn assert_zero(&mut self, value: Cell) {
        self.constraint |= value.0;
    }
}

struct FilteredBuilder<'a, B> {
    inner: &'a mut B,
    condition: Cell,
}

impl<B: ConstraintBuilder> ConstraintBuilder for FilteredBuilder<'_, B> {
    type Var = B::Var;

    fn assert_zero(&mut self, value: Cell) {
        self.inner
            .assert_zero(Cell(self.condition.0.wrapping_mul(value.0)));
    }
}

fn constrain_filtered<B: ConstraintBuilder>(builder: &mut B, value: Cell) {
    let gate = Cell(1);
    let builder = &mut builder.when(gate);
    let expected = Cell(value.0.wrapping_add(1));
    builder.assert_eq(value, expected);
    builder.assert_bool(value);
}

fn constrain_generic_input<B: ConstraintBuilder>(builder: &mut B, cells: &Cells<B::Var>) {
    constrain_filtered(builder, cells.values[0].into());
}

fn constrain_generic_row<B: ConstraintBuilder>(builder: &mut B, row: &Row<B::Var>) {
    constrain_generic_input(builder, row.middle());
}


fn constrain_generic_loop<B: ConstraintBuilder>(builder: &mut B, cells: &Cells<B::Var>) {
    let builder = &mut builder.when(Cell(1));
    for value in cells.values {
        builder.assert_zero(value.into());
    }
}

fn evaluate(builder: &mut Builder<'_>) {
    let main = builder.main();
    constrain_generic_row(builder, borrow_row(main.current_slice()));
}

pub fn main() {
    let input = Cells {
        values: [Cell(0), Cell(1)],
    };
    add_tag!(&input.values[0].0, InputTag);
    let output = input.copied();
    verify!(has_tag!(&output[0].0, InputTag));

    let padded = [Cell(0), Cell(1), Cell(2), Cell(3)];
    add_tag!(&padded[2].0, InputTag);
    let subslice: &Cells<Cell> = padded[1..3].borrow();
    let output = subslice.copied();
    verify!(!has_tag!(&output[0].0, InputTag));
    verify!(has_tag!(&output[1].0, InputTag));

    let padded = PaddedCells {
        values: [Cell(0), Cell(1), Cell(2), Cell(3)],
    };
    add_tag!(&padded.values[2].0, InputTag);
    let output = padded.middle().copied();
    verify!(!has_tag!(&output[0].0, InputTag));
    verify!(has_tag!(&output[1].0, InputTag));

    let row = [Cell(0), Cell(1), Cell(2), Cell(3), Cell(4)];
    add_tag!(&row[2].0, InputTag);
    let output = borrow_row(&row).middle().copied();
    verify!(!has_tag!(&output[0].0, InputTag));
    verify!(has_tag!(&output[1].0, InputTag));

    let builder_row = [Cell(0), Cell(1), Cell(2), Cell(3), Cell(4)];
    add_tag!(&builder_row[1].0, InputTag);
    let mut builder = Builder {
        main: RowWindow {
            current: &builder_row,
            next: &builder_row,
        },
        constraint: 0,
    };
    evaluate(&mut builder);
    verify!(has_tag!(&builder.constraint, InputTag));

    let loop_cells = Cells {
        values: [Cell(0), Cell(0)],
    };
    add_tag!(&loop_cells.values[0].0, InputTag);
    builder.constraint = 0;
    constrain_generic_loop(&mut builder, &loop_cells);
    verify!(has_tag!(&builder.constraint, InputTag));
}
