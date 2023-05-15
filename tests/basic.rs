use c_enum::*;

c_enum! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Hardware : u64 {
        /// Doc comments
        CPU_CYCLES,
        INSTRUCTIONS = 2,
        CACHE_REFERENCES,
        CACHE_MISSES,
        BRANCH_INSTRUCTIONS = 5,
        Lowercase,
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

c_enum! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Duplicates : u8 {
        ITEM1 = 2,
        ITEM2 = 2
    }
}
