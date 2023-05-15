
#[macro_export]
macro_rules! c_enum {
    {
        $( #[$attr:meta] )*
        $vis:vis enum $name:ident : $inner:ty {
            $(
                $( #[ $field_attr:meta ] )*
                $field:ident $( = $value:expr )?
            ),* $(,)?
        }
    } => {
        $crate::c_enum_no_debug! {
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
                match self {
                    $( 
                        Self(value) if $name::$field.0 == *value => f.write_str(
                            ::core::concat!(
                                ::core::stringify!($name),
                                "::",
                                ::core::stringify!($field)
                            )
                        ),
                    )*
                    Self(value) => f
                        .debug_tuple(::core::stringify!($name))
                        .field(value)
                        .finish()
                }
            }
        }
    };
}

#[macro_export]
macro_rules! c_enum_no_debug {
    {
        $( #[$attr:meta] )*
        $vis:vis enum $name:ident : $inner:ty {
            $(
                $( #[ $field_attr:meta ] )*
                $field:ident $( = $value:expr )?
            ),* $(,)?
        }
    } => {
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

        impl $name {
            #[doc = concat!(
                "Create a new `",
                stringify!($name),
                "` from a `",
                stringify!($inner),
                "`."
            )]
            #[inline]
            pub const fn new(value: $inner) -> Self {
                Self(value)
            }
        }
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

c_enum_no_debug! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum Software : u64 {
        /// Doc comments
        CPU_CYCLES,
        INSTRUCTIONS = 2,
        CACHE_REFERENCES,
        CACHE_MISSES,
        BRANCH_INSTRUCTIONS = 5,
        Lowercase,
    }
}
