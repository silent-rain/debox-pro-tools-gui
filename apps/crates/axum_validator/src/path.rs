//! Path extractor.

use axum::{extract::FromRequestParts, http::request::Parts};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::Error;

/// A wrapper around `Axum Path` that validates the extracted path parameters.
#[derive(Debug)]
pub struct Path<T>(pub T);

impl<T, S> FromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Validate + Send,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let path =
            <axum::extract::Path<T> as axum::extract::FromRequestParts<S>>::from_request_parts(
                parts, state,
            )
            .await
            .map_err(|e| Error::PathRejection(e.to_string()))?;
        path.0.validate()?;
        Ok(Path(path.0))
    }
}
