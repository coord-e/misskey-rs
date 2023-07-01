pub mod create;
pub mod featured;
pub mod follow;
pub mod followed;
pub mod owned;
pub mod show;
pub mod timeline;
pub mod unfollow;
pub mod update;

#[cfg(feature = "13-11-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-11-0")))]
pub mod favorite;

#[cfg(feature = "13-11-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-11-0")))]
pub mod my_favorites;

#[cfg(feature = "13-11-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-11-0")))]
pub mod unfavorite;

#[cfg(feature = "13-11-2")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-11-2")))]
pub mod search;
