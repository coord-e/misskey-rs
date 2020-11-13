//! Object types used in API.

macro_rules! impl_entity {
    ($name:ident) => {
        impl crate::Entity for $name {
            fn id(&self) -> crate::model::id::Id<$name> {
                self.id
            }
        }
    };
}

pub mod abuse_user_report;
pub mod announcement;
pub mod antenna;
pub mod blocking;
pub mod channel;
pub mod chart;
pub mod clip;
pub mod drive;
pub mod emoji;
pub mod following;
pub mod id;
pub mod log;
pub mod messaging;
pub mod muting;
pub mod note;
pub mod note_favorite;
pub mod notification;
pub mod page;
pub mod signin;
pub mod sort;
pub mod user;
pub mod user_group;
pub mod user_list;
