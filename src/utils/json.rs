//! # JSON (Sonic)
//!
use axum_serde::extractor;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sonic_rs::Error;

extractor!(
    JSON,
    Json,
    "application/json",
    from_slice,
    Error,
    to_vec,
    sonic_test
);

fn from_slice<T: DeserializeOwned>(s: &[u8]) -> Result<T, Error> {
    // let src = std::str::from_utf8(s).map_err(Error::custom)?;
    sonic_rs::from_slice(s)
}

fn to_vec<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    // let s = sonic_rs::to_string(value)?;
    // Ok(s.into_bytes())
    sonic_rs::to_vec(value)
}

#[cfg(test)]
mod tests {
    use axum::response::IntoResponse;
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct Person {
        name: String,
    }
    #[test]
    fn test_name() {
        let person = Person {
            name: "Alice".to_string(),
        };
        
        let data = Json(person);
        let resp = data.into_response();
        assert_eq!(resp.status(), 200);
        assert_eq!(resp.headers().get("content-type").unwrap(), "application/json");
        
    
    }
}