pub mod create;
pub mod delete;
pub mod like;
pub mod show;
pub mod unlike;
pub mod update;

#[cfg(feature = "12-58-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-58-0")))]
pub mod featured;
