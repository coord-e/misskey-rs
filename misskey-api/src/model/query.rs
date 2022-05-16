use crate::model::note::Tag;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(transparent)]
pub struct Query<T>(pub Vec<Vec<T>>);

impl<T> Query<T> {
    /// Creates an empty [`Query`].
    pub fn new() -> Self {
        Query(vec![])
    }

    /// Creates [`Query`] from a vector which represents disjunction of conjunctions of `T`.
    pub fn from_vec(vec: Vec<Vec<T>>) -> Self {
        Query(vec)
    }

    /// Turns [`Query`] into a vector which represents disjunction of conjunctions of `T`.
    pub fn into_vec(self) -> Vec<Vec<T>> {
        self.0
    }

    /// Creates [`Query`] from a single element.
    pub fn atom(x: impl Into<T>) -> Self {
        Query(vec![vec![x.into()]])
    }

    /// Disjunction of two queries.
    pub fn or(mut self, rhs: impl Into<Self>) -> Self {
        let mut rhs = rhs.into();
        self.0.append(&mut rhs.0);
        self
    }

    /// Conjunction of two queries. This operation is expensive for large rhs
    /// and implemented just for syntactic brevity of small queries.
    ///
    /// For building larger queries, consider using [`Query::or`] or [`Query::from_vec`].
    pub fn and(mut self, rhs: impl Into<Self>) -> Self
    where
        T: Clone,
    {
        let rhs: Self = rhs.into();
        let mut result = Vec::new();
        for and_lhs in self.0.drain(..) {
            for and_rhs in rhs.0.clone() {
                let mut and = and_lhs.clone();
                and.extend(and_rhs);
                result.push(and);
            }
        }
        Query(result)
    }
}

impl<T> From<T> for Query<T> {
    fn from(x: T) -> Query<T> {
        Query(vec![vec![x]])
    }
}

// we can't `impl<T, U> From<T> for Query<U> where T: Into<U>`
// however these `impl`s cover most use case

impl From<&str> for Query<String> {
    fn from(x: &str) -> Query<String> {
        Query(vec![vec![x.to_string()]])
    }
}

impl From<&str> for Query<Tag> {
    fn from(x: &str) -> Query<Tag> {
        Query(vec![vec![Tag(x.to_string())]])
    }
}
