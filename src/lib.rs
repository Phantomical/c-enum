//! A macro for easily defining a c-like rust enum.
//!
//! The [`c_enum!`] macro generates structs that behave roughly like a C enum:
//! they have a set of constants that have integer values but can be assigned
//! integer values that don't correspond to any of the existing names.
//!
//! # Examples
//! ```
//! use c_enum::c_enum;
//!
//! c_enum! {
//!     #[derive(Copy, Clone, PartialEq, Eq, Hash)]
//!     pub enum MyEnum: u32 {
//!         A,
//!         B = 5,
//!     }
//! }
//!
//! fn main() {
//!     let v1 = MyEnum::A;       // declared variant
//!     let v2 = MyEnum::from(3); // also supports variants that are not declared
//!
//!     match v1 { // we can match if we derive PartialEq
//!         MyEnum::A => println!("got an A"),
//!         MyEnum::B => println!("got a B"),
//!
//!         // We still need to handle other variants
//!         _ => println!("got another variant"),
//!     }
//! }
//! ```
//!
//! # Visibility
//! The `c_enum!` macro supports visibility, just like you would do for a normal
//! rust enum.
//!
//! ```
//! # #[macro_use]
//! # extern crate c_enum;
//! #
//! mod example {
//!     c_enum! {
//!         pub enum Enum1: u8 {
//!             A,
//!         }
//!
//! #       pub
//!         enum Enum2: u8 {
//!             B,
//!         }
//!     }
//! }
//!
//! # fn main() {
//! let val1 = example::Enum1::A;
//! let val2 = example::Enum2::B; // error: struct `Enum2` is private
//! # }
//! ```
//!
//! # Attributes
//! Attributes can be added to the generated type or variants as normal. Note
//! that the variants are converted to constants so macros expecting an enum
//! variant will not work.
//!
//! # Representation
//! It is valid to add a `#[repr(C)]` or `#[repr(transparent)]` attribute to the
//! generated type. The generated type is guaranteed to be a newtype whose only
//! member is the inner type.
//!
//! # Value Assignment
//! By default, enum values are assigned like they would be for a C enum: the
//! first variant is 0 and subsequent variants increase by 1 unless assigned a
//! value.
//!
//! ```
//! # #[macro_use]
//! # extern crate c_enum;
//! #
//! c_enum! {
//!     pub enum Enum: u32 {
//!         A,     // value of 0
//!         B,     // value of 1
//!         C = 5, // value of 5
//!         D,     // value of 6
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! ## Non-String Inner Types
//! It is also possible to define enum types whose inner value is not an
//! integer.
//!
//! ```
//! # #[macro_use]
//! # extern crate c_enum;
//! #
//! c_enum! {
//!     pub enum StringEnum: &'static str {
//!         Hello = "Hello",
//!         World = "World",
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! Note that at this time generics are not supported so any inner value type
//! must be both concrete and `'static`. Furthermore, you will need to assign a
//! value to each variant of such an enum.
//!
//! # What's implemented by `c_enum!`
//! The [`c_enum!`] macro implements some traits by default, but leaves the rest
//! available for you to choose the semantics of the rest.
//!
//! - [`CEnum`] which contains some common methods for all c-like enums.
//!
//! ## Formatting
//! - [`Debug`], but only if the inner type implements [`PartialEq`] and
//!   [`Debug`].
//!
//! ## Conversion
//! - [`From`] to convert from the inner type and vice versa.
//!
//! # Non-Integer Enums
//! Creating
//!
//! # Generated Code
//! ```
//! # #[macro_use]
//! # extern crate c_enum;
//! #
//! c_enum! {
//!     #[repr(transparent)]
//!     #[derive(Copy, Clone, PartialEq, Eq, Hash)]
//!     enum Enum: u32 {
//!         A,
//!         B = 5,
//!     }
//! }
//! # fn main() {}
//! ```
//! is expanded into (roughly)
//! ```
//! # macro_rules! ignore { {$( $tt:tt )*} => {} }
//! # #[macro_use]
//! # extern crate c_enum;
//! #
//! #[repr(transparent)]
//! #[derive(Copy, Clone, PartialEq, Eq, Hash)]
//! struct Enum(u32);
//!
//! impl Enum {
//!     pub const A: Self = Self(0);
//!     pub const B: Self = Self(5);
//! }
//!
//! # ignore! {
//! impl core::fmt::Debug for Enum
//! where
//!     u32: core::cmp::PartialEq
//! {
//!     ...
//! }
//!
//! // more trait impls...
//! # }
//! # fn main() {}
//! ```
//!
//! # Motivation
//! When writing bindings for C libraries which use enums there are a few
//! options for declaring rust versions of C enums.
//! - Use a rust enum.
//! - Use a raw integer and a bunch of constants.
//! - Use a newtype and a set of constants.
//!
//! All of them have use cases for which they are valid:
//! - Rust enums work when calling from rust code into C code. The one caveat
//!   here being that if the underlying C library adds a new variant but the
//!   rust wrapper does not then users of the rust library are stuck. Another
//!   case that is valid is if it is known that no new variants will be added to
//!   the underlying C enum and the library is ok with either UB or doing
//!   conversions at the API boundary.
//! - Raw integers and constants is useful for autogenerated bindings that want
//!   to exactly match the layout of the C headers.
//! - A newtype + constants is suitable for the remaining cases. It still
//!   behaves similar to a rust enum but matches the actual semantics of C
//!   enums. It also continues to work if the C library adds new variants and
//!   the rust wrapper is not updated.
//!
//! This crate is a generator for the third option.
//!
//! [`Debug`]: core::fmt::Debug
//! [`PartialEq`]: core::cmp::PartialEq

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate self as c_enum;

/// A trait that is automatically implemented for all C enums.
pub trait CEnum: From<Self::Inner> + Into<Self::Inner> {
    /// The inner type of this enum.
    type Inner;

    /// Get the string name corresponding to the current value, if there is one.
    fn variant_label(&self) -> Option<&'static str>
    where
        Self::Inner: PartialEq;
}

/// The macro used to generate the C enum structure.
///
/// See the [crate level docs](crate) for complete documentation.
#[macro_export]
macro_rules! c_enum {
    {
        $(
            $( #[$attr:meta] )*
            $vis:vis enum $name:ident : $inner:ty {
                $(
                    $( #[ $field_attr:meta ] )*
                    $field:ident $( = $value:expr )?
                ),* $(,)?
            }
        )+
    } => {
        $(
            $crate::__c_enum_no_debug! {
                $( #[$attr] )*
                $vis enum $name : $inner {
                    $(
                        $( #[ $field_attr ] )*
                        $field $( = $value )?
                    ),*
                }
            }

            impl ::core::fmt::Debug for $name
            where
                $inner: ::core::fmt::Debug,
                $inner: ::core::cmp::PartialEq
            {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>
                ) -> ::core::fmt::Result {
                    use $crate::CEnum;

                    match self.variant_label() {
                        Some(variant) => {
                            f.write_fmt(::core::format_args!(
                                "{}::{}", ::core::stringify!($name), variant
                            ))
                        },
                        None => f
                            .debug_tuple(::core::stringify!($name))
                            .field(&self.0)
                            .finish()
                    }
                }
            }
        )+
    };
}

// TODO: not sure if this is worth adding to the public API.
/// The macro used to generate the C enum structure.
///
/// This version does not generate a [`Debug`] impl.
///
/// See the [crate level docs](crate) for complete documentation.
///
/// [`Debug`]: core::fmt::Debug
#[macro_export]
#[doc(hidden)]
macro_rules! __c_enum_no_debug {
    {
        $(
            $( #[$attr:meta] )*
            $vis:vis enum $name:ident : $inner:ty {
                $(
                    $( #[ $field_attr:meta ] )*
                    $field:ident $( = $value:expr )?
                ),* $(,)?
            }
        )+
    } => {
        $(
            $( #[$attr] )*
            $vis struct $name(pub $inner);

            #[allow(non_upper_case_globals)]
            impl $name {
                $crate::__c_enum_impl!(
                    impl(decl_variants, $name, $inner)
                    [ $(
                        $( #[$field_attr] )*
                        $field $( = $value )?,
                    )*]
                    [
                        __dummy = 0,
                        $( $field $( = $value )?, )*
                    ]
                );
            }

            impl From<$inner> for $name {
                fn from(value: $inner) -> Self {
                    Self(value)
                }
            }

            impl From<$name> for $inner {
                fn from(value: $name) -> Self {
                    value.0
                }
            }

            impl $crate::CEnum for $name {
                type Inner = $inner;

                fn variant_label(&self) -> Option<&'static str>
                where
                    Self::Inner: PartialEq
                {
                    Some(match &self.0 {
                        $( value if Self::$field.0 == *value => ::core::stringify!($name), )*
                        _ => return None,
                    })
                }
            }
        )+
    };
}

/// Helper macro for defining stuff in c_enum.
///
/// These could be a bunch of different macros but those would clutter up the
/// import namespace when using something like rust-analyzer. By using a single
/// internal macro we can avoid that.
#[doc(hidden)]
#[macro_export]
macro_rules! __c_enum_impl {
    (impl(first_expr) $first:expr $( , $rest:expr )*) => {
        $first
    };

    (
        impl(decl_variants, $name:ident, $inner:ty)
        [ ]
        [ $( $__:ident $( = $prev:expr )? ),* $(,)? ]
    ) => {};
    (
        impl(decl_variants, $name:ident, $inner:ty)
        [
            $( #[$fattr:meta] )*
            $field:ident $( = $fvalue:expr )?
            $( ,
                $( #[$rattr:meta] )*
                $frest:ident $( = $frest_val:expr )?
            )*
            $(,)?
        ]
        [ $prev:ident  $( = $pvalue:expr )? $( , $prest:ident $( = $prest_val:expr )? )* $(,)? ]
    ) => {
        $( #[$fattr] )*
        #[allow(non_upper_case_globals)]
        pub const $field: Self = Self($crate::__c_enum_impl!(impl(first_expr) $( $fvalue, )? $( $pvalue, )? 0));

        $crate::__c_enum_impl!(
            impl(decl_variants, $name, $inner)
            [ $( $( #[$rattr] )* $frest $( = $frest_val )?, )* ]
            [ $( $prest $( = $prest_val )?, )* ]
        );
    }
}

// This needs to be after all the macro definitions.
/// This module shows an example of code generated by the macro.
///
/// The source code of this module is
/// ```
#[doc = include_str!("example.rs")]
/// ```
#[cfg(doc)]
#[cfg_attr(docsrs, doc(cfg(doc)))]
pub mod example;
