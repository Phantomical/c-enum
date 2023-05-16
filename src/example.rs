use c_enum::c_enum;

c_enum! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Enum : u64 {
        DefaultVariant,

        /// We can also put doc comments and other attributes on a variant.
        DocumentedVariant,

        /// Or, we can assign values to a variant.
        VariantWithValue = 0x777,

        /// We can also refer to the values of other variants.
        VariantWithComputedValue = Self::DefaultVariant.0 + 7,
    }
}
