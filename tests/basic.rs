use c_enum::*;

c_enum! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

c_enum! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Duplicates : u8 {
        ITEM1 = 2,
        ITEM2 = 2
    }
}

#[test]
fn increment_after_assign() {
    assert_eq!(Software::Lowercase.0, 6);
    assert_ne!(Software::Lowercase, Software::BRANCH_INSTRUCTIONS);
}

#[test]
fn duplicates_are_equal() {
    assert_eq!(Duplicates::ITEM1, Duplicates::ITEM2);
}

#[test]
fn verify_variant_label() {
    assert_eq!(Software::CPU_CYCLES.variant_label(), Some("CPU_CYCLES"));
}

#[test]
fn variant_label_duplicate() {
    // In the case of duplicates the variant that comes first should be the
    // the one whose label is used.
    assert_eq!(Duplicates::ITEM1.variant_label(), Some("ITEM1"));
    assert_eq!(Duplicates::ITEM2.variant_label(), Some("ITEM1"));
}

#[test]
fn variant_label_overlap_assigned() {
    c_enum! {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        enum Overlap : u32 {
            A1 = 2,
            A2,
            A3,
            B1 = 2,
            B2,
            B3
        }
    }

    assert_eq!(Overlap::A1, Overlap::B1);
    assert_eq!(Overlap::A3, Overlap::B3);
}
