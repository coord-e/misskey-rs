pub mod add;
pub mod copy;
pub mod list;
pub mod list_remote;
pub mod update;

#[cfg(not(feature = "12-102-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "12-102-0"))))]
pub mod remove;

#[cfg(feature = "12-102-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-102-0")))]
pub mod add_aliases_bulk;
#[cfg(feature = "12-102-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-102-0")))]
pub mod delete;
#[cfg(feature = "12-102-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-102-0")))]
pub mod delete_bulk;
#[cfg(feature = "12-102-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-102-0")))]
pub mod import_zip;
#[cfg(feature = "12-102-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-102-0")))]
pub mod remove_aliases_bulk;
#[cfg(feature = "12-102-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-102-0")))]
pub mod set_aliases_bulk;
#[cfg(feature = "12-102-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-102-0")))]
pub mod set_category_bulk;
