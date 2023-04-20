
pub type Equal<const A: i32, const B: i32> = crate::Check<{ A == B }>;