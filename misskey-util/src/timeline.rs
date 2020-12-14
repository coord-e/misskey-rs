use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

use chrono::{DateTime, Utc};
use derivative::Derivative;
use misskey_api::{model::id::Id, Entity};

/// Range in the timeline.
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
#[derivative(PartialEq(bound = ""), Eq(bound = ""))]
#[derivative(Clone(bound = ""), Copy(bound = ""))]
pub enum TimelineRange<E> {
    /// Range in the timeline bounded by time.
    DateTime {
        /// The lower bound of the range (inclusive), if it exists.
        since_date: Option<DateTime<Utc>>,
        /// The upper bound of the range (exclusive), if it exists.
        until_date: Option<DateTime<Utc>>,
    },
    /// Range in the timeline bounded by note IDs.
    Id {
        /// The lower bound of the range (inclusive), if it exists.
        since_id: Option<Id<E>>,
        /// The upper bound of the range (exclusive), if it exists.
        until_id: Option<Id<E>>,
    },
    /// Unbounded range.
    Unbounded,
}

impl<E> TimelineRange<E> {
    /// Returns [`TimelineRange`] for the range to a specified point on the timeline.
    pub fn until(cursor: TimelineCursor<E>) -> Self {
        match cursor {
            TimelineCursor::Id(id) => TimelineRange::Id {
                since_id: None,
                until_id: Some(id),
            },
            TimelineCursor::DateTime(date) => TimelineRange::DateTime {
                since_date: None,
                until_date: Some(date),
            },
        }
    }

    /// Returns [`TimelineRange`] for the range from a specified point on the timeline.
    pub fn since(cursor: TimelineCursor<E>) -> Self {
        match cursor {
            TimelineCursor::Id(id) => TimelineRange::Id {
                since_id: Some(id),
                until_id: None,
            },
            TimelineCursor::DateTime(date) => TimelineRange::DateTime {
                since_date: Some(date),
                until_date: None,
            },
        }
    }
}

impl<E: Entity> From<RangeFull> for TimelineRange<E> {
    fn from(RangeFull: RangeFull) -> Self {
        TimelineRange::Unbounded
    }
}

// We can't impl<E: Entity, R: EntityRef<E>> From<$range<R>> for TimelineRange<E>
// because impl for DateTime<Utc> and R conflicts
macro_rules! impl_from_range {
    ($range:ident, $arg:ident, $since:expr, $until:expr) => {
        impl<E: Entity> From<$range<Id<E>>> for TimelineRange<E> {
            fn from($arg: $range<Id<E>>) -> Self {
                TimelineRange::Id {
                    since_id: $since,
                    until_id: $until,
                }
            }
        }

        impl<E: Entity> From<$range<&E>> for TimelineRange<E> {
            fn from($arg: $range<&E>) -> Self {
                TimelineRange::Id {
                    since_id: $since.map(Entity::id),
                    until_id: $until.map(Entity::id),
                }
            }
        }

        impl<E: Entity> From<$range<DateTime<Utc>>> for TimelineRange<E> {
            fn from($arg: $range<DateTime<Utc>>) -> Self {
                TimelineRange::DateTime {
                    since_date: $since,
                    until_date: $until,
                }
            }
        }
    };
}

impl_from_range! { Range, range, Some(range.start), Some(range.end) }
impl_from_range! { RangeFrom, range, Some(range.start), None }
impl_from_range! { RangeTo, range, None, Some(range.end) }

/// Point on the timeline.
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
#[derivative(PartialEq(bound = ""), Eq(bound = ""))]
#[derivative(Clone(bound = ""), Copy(bound = ""))]
pub enum TimelineCursor<E> {
    /// Point on the timeline specified by time.
    DateTime(DateTime<Utc>),
    /// Point on the timeline specified by note ID.
    Id(Id<E>),
}

// We can't impl<E: Entity, R: EntityRef<E>> From<R> for TimelineCursor<E>
// because impl for DateTime<Utc> and R conflicts
impl<E: Entity> From<DateTime<Utc>> for TimelineCursor<E> {
    fn from(time: DateTime<Utc>) -> Self {
        TimelineCursor::DateTime(time)
    }
}

impl<E: Entity> From<Id<E>> for TimelineCursor<E> {
    fn from(id: Id<E>) -> Self {
        TimelineCursor::Id(id)
    }
}

impl<E: Entity> From<&E> for TimelineCursor<E> {
    fn from(entity: &E) -> Self {
        TimelineCursor::Id(entity.id())
    }
}
