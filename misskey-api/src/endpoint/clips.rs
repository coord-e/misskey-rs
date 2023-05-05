pub mod create;
pub mod delete;
pub mod list;
pub mod notes;
pub mod show;
pub mod update;

#[cfg(feature = "12-57-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-57-0")))]
pub mod add_note;

#[cfg(feature = "12-112-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
pub mod remove_note;

#[cfg(feature = "13-10-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
pub mod favorite;

#[cfg(feature = "13-10-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
pub mod my_favorites;

#[cfg(feature = "13-10-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
pub mod unfavorite;
