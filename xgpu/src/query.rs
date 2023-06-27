//! GPU queries.

pub struct QuerySet {
    // TODO
}

impl QuerySet {
    pub fn kind(&self) -> QueryKind {
        panic!("not yet implemented");
    }

    pub fn count(&self) -> u32 {
        panic!("not yet implemented");
    }
}

pub struct QuerySetDescriptor {
    pub kind: QueryKind,
    pub count: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum QueryKind {
    Occlusion,
    Timestamp,
}
