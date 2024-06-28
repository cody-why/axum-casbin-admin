use crate::{error::Error, service::context};
use axum::extract::FromRequestParts;
use axum::http::header;
use axum::http::request::Parts;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{sync::OnceLock, time::Duration};

use super::get_timestamp;

fn get_key() -> &'static Keys {
    static KEYS: OnceLock<Keys> = OnceLock::new();
    KEYS.get_or_init(|| {
        let secret = context().config.jwt_secret.as_str();
        Keys::new(secret)
    })
}

struct Keys {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl Keys {
    fn new(secret: impl AsRef<[u8]>) -> Self {
        let secret = secret.as_ref();
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTToken {
    pub id: u64,
    pub username: String,
    pub permissions: Vec<String>,
    exp: u64,
    iat: u64,
    // aud: String,
}

impl JWTToken {
    pub fn new(id: u64, username: &str, permissions: Vec<String>) -> JWTToken {
        //过期时间
        let m30 = Duration::from_secs(context().config.jwt_exp).as_secs();
        let now = get_timestamp();

        JWTToken {
            id,
            username: String::from(username),
            permissions,
            exp: now + m30,
            iat: now, /* (Issued At)：签发时间
                       * aud: String::from("rust_admin"), // (audience)：受众
                       * iss: String::from("code"),     // (issuer)：签发人
                       * nbf: now.as_secs() as usize,  // (Not Before)：生效时间
                       * sub: String::from("rust_admin"), // (subject)：主题
                       * jti: String::from("ignore"),  // (JWT ID)：编号 */
        }
    }

    /// create token
    pub fn create_token(&self) -> Result<String, Error> {
        encode(
            &Header::default(),
            self,
            &get_key().encoding_key,
        )
        .map_err(Error::Jwt)
    }

    /// verify token invalid
    pub fn verify(token: &str) -> Result<JWTToken, Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        // validation.sub = Some("rust_admin".to_string());
        // validation.set_audience(&["rust_admin"]);
        validation.set_required_spec_claims(&["exp"]); // "aud"

        decode::<JWTToken>(
            token,
            &get_key().decoding_key,
            &validation,
        )
        .map(|c| c.claims)
        .map_err(Error::Jwt)
    }

    pub fn check_refresh(&mut self) -> Result<String, Error> {
        let now = get_timestamp();
        if context().config.jwt_refresh_token > 0 && now > self.iat + context().config.jwt_refresh_token {
            self.exp = now + context().config.jwt_exp;
            self.iat = now;
            return self.create_token();
        }
        Err("no need refresh token".into())
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for JWTToken
where
    S: Send + Sync,
{
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // info!("JWTToken from_request_parts");
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok());

        let token = if let Some(auth_header) = auth_header {
            auth_header.to_string().replace("Bearer ", "")
        } else {
            // info!("Authorization header not found");
            return Err("Authorization header not found".into());
        };

        // info!("token:{}",token);
        let jwt_token_e = JWTToken::verify(&token);
        let jwt_token = match jwt_token_e {
            Ok(data) => data,
            Err(err) => {
                // error!("Token verify error:{}",err);
                return Err(format!(
                    "Token verify error:{}",
                    err
                ));
            },
        };
        Ok(jwt_token)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::jwt_util::JWTToken;

    #[test]
    fn test_jwt() {
        let jwt = JWTToken::new(1, "code", vec![]);
        let res = jwt.create_token();
        println!("{:?}", res);
        let token = JWTToken::verify(&res.unwrap());
        println!("{:?}", token)
    }
}
