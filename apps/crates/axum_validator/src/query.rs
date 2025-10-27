//! Query extractor.

use std::ops::Deref;

use axum::{extract::FromRequestParts, http::request::Parts};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::Error;

/// A wrapper around `Axum Query` that validates the extracted path parameters.
#[derive(Debug)]
pub struct Query<T>(pub T);

impl<T> Query<T> {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for Query<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T, S> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned + Validate + Send,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let query =
            <axum::extract::Query<T> as axum::extract::FromRequestParts<S>>::from_request_parts(
                parts, state,
            )
            .await
            .map_err(|e| Error::QueryRejection(e.to_string()))?;
        query.0.validate()?;
        Ok(Query(query.0))
    }
}
