//! Utilities for working with `misskey-rs`.
//!
//! This crate wraps [`misskey-api`][misskey_api] to implement a convenient high-level API.
//!
//! [misskey_api]: https://docs.rs/misskey_api

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! update_builder_option_field {
    (
        #[doc_name = $doc_name:tt]
        $(#[$m:meta])*
        $v:vis $name:ident : $param_type:ty { $field_name:ident = $value:expr };
        $($tail:tt)*
    ) => {
        paste::paste! {
            #[doc = "Sets the " $doc_name "."]
            $(#[$m])*
            $v fn [<set_ $name>](&mut self, $name: $param_type) -> &mut Self {
                self.request.$field_name.replace(Some($value));
                self
            }

            #[doc = "Deletes the " $doc_name "."]
            $(#[$m])*
            $v fn [<delete_ $name>](&mut self) -> &mut Self {
                self.request.$field_name.replace(None);
                self
            }

            update_builder_option_field! { $($tail)* }
        }
    };
    (
        #[doc_name = $doc_name:tt]
        $(#[$m:meta])*
        $v:vis $name:ident : $param_type:ty { $field_name:ident };
        $($tail:tt)*
    ) => {
        update_builder_option_field! {
            #[doc_name = $doc_name]
            $(#[$m])*
            $v $name: $param_type { $field_name = ($field_name) };
        }
        update_builder_option_field! { $($tail)* }
    };
    (
        $(#[$m:meta])*
        $v:vis $name:ident $($tail:tt)*
    ) => {
        update_builder_option_field! { #[doc_name = $name] $(#[$m])* $v $name $($tail)* }
    };
    () => {};
}

macro_rules! update_builder_string_collection_field {
    (
        $(#[$m:meta])*
        $v:vis $name:ident { $field_name:ident };
        $($tail:tt)*
    ) => {
        $(#[$m])*
        $v fn $name(&mut self, $name: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
            self.request
                .$field_name
                .replace($name.into_iter().map(Into::into).collect());
            self
        }
        update_builder_string_collection_field! { $($tail)* }
    };
    (
        $(#[$m:meta])*
        $v:vis $name:ident;
        $($tail:tt)*
    ) => {
        update_builder_string_collection_field! {
            $(#[$m])*
            $v $name { $name };
        }
        update_builder_string_collection_field! { $($tail)* }
    };
    () => {};
}

macro_rules! update_builder_string_option_field {
    (
        #[doc_name = $doc_name:tt]
        $(#[$m:meta])*
        $v:vis $name:ident { $field_name:ident };
        $($tail:tt)*
    ) => {
        update_builder_option_field! {
            #[doc_name = $doc_name]
            $(#[$m])*
            $v $name: impl Into<String> { $field_name = $name.into() };
        }
        update_builder_string_option_field! { $($tail)* }
    };
    (
        $(#[$m:meta])*
        $v:vis $name:ident { $field_name:ident };
        $($tail:tt)*
    ) => {
        update_builder_option_field! {
            $(#[$m])*
            $v $name: impl Into<String> { $field_name = $name.into() };
        }
        update_builder_string_option_field! { $($tail)* }
    };
    (
        #[doc_name = $doc_name:tt]
        $(#[$m:meta])*
        $v:vis $name:ident;
        $($tail:tt)*
    ) => {
        update_builder_string_option_field! {
            #[doc_name = $doc_name]
            $(#[$m])*
            $v $name { $name };
        }
        update_builder_string_option_field! { $($tail)* }
    };
    (
        $(#[$m:meta])*
        $v:vis $name:ident;
        $($tail:tt)*
    ) => {
        update_builder_string_option_field! {
            $(#[$m])*
            $v $name { $name };
        }
        update_builder_string_option_field! { $($tail)* }
    };
    () => {};
}

macro_rules! update_builder_bool_field {
    (
        $(#[$m:meta])*
        $v:vis $name:ident { $field_name:ident };
        $($tail:tt)*
    ) => {
        $(#[$m])*
        $v fn $name(&mut self, $name: bool) -> &mut Self {
            self.request.$field_name.replace($name);
            self
        }
        update_builder_bool_field! { $($tail)* }
    };
    (
        $(#[$m:meta])*
        $v:vis $name:ident;
        $($tail:tt)*
    ) => {
        update_builder_bool_field! {
            $(#[$m])*
            $v $name { $name };
        }
        update_builder_bool_field! { $($tail)* }
    };
    () => {};
}

mod error;
pub use error::Error;

mod client;
pub use client::{ClientExt, UploadFileClientExt};

pub mod builder;
pub mod pager;

mod timeline;
pub use timeline::{TimelineCursor, TimelineRange};
