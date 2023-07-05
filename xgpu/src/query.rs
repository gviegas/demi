//! GPU queries.

pub struct QuerySet {
    kind: QueryKind,
    count: u32,
    // TODO: `state`.
    // TODO
}

impl QuerySet {
    pub fn kind(&self) -> QueryKind {
        self.kind
    }

    pub fn count(&self) -> u32 {
        self.count
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query() {
        // TODO: `QuerySet::new`.
        let qset = QuerySet {
            kind: QueryKind::Occlusion,
            count: 128,
        };
        let _ = qset.kind();
        let _ = qset.count();
    }
}
