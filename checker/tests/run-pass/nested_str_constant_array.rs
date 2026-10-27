// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use mirai_annotations::{assume, verify};
use std::borrow::Cow;

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Event {
    First,
    Second,
}

#[derive(Copy, Clone)]
struct EventEntry {
    id: EventId,
    event: Event,
    name: &'static str,
}

#[derive(Copy, Clone)]
struct EventId(u64);

impl Event {
    const LOOKUP: [EventEntry; 25] = [
        EventEntry { id: EventId(u64::MAX), event: Event::First, name: "sys::first" };
        25
    ];

    pub fn event_name(&self) -> EventName {
        let index = *self as usize;
        assume!(index < Self::LOOKUP.len());
        EventName::new(Self::LOOKUP[index].name)
    }
}

pub struct EventName(Cow<'static, str>);

impl EventName {
    pub const fn new(name: &'static str) -> Self {
        Self(Cow::Borrowed(name))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<Event> for EventName {
    fn from(event: Event) -> Self {
        event.event_name()
    }
}

pub fn main() {
    let entry = Event::LOOKUP[0];
    verify!(entry.id.0 == u64::MAX);
    verify!(entry.event as u8 == Event::First as u8);

    let name = EventName::from(Event::First);
    verify!(name.as_str() == "sys::first");
}
