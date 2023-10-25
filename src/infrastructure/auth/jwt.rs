use std::{backtrace::Backtrace, time::Instant};

use crate::utils::{hex, hex_literal};
use base64::{base64_decode, base64url_encode};
use hmac::{Hmac, Mac};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{from_str, json};
use sha2::Sha256;
use subtle::ConstantTimeEq;
use tracing::debug;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug)]
pub enum JwtError {
    EncodingError,
    ParseJwtError(Backtrace),
    ParseHeadersError,
    ParseClaimsError,
    VerificationError,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Algorithm {
    #[default]
    HS256,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum JwtType {
    #[default]
    JWT,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JwtHeaders {
    alg: Algorithm,
    typ: JwtType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Jwt<'a, Claims> {
    secret: &'a str,
    headers: JwtHeaders,
    pub claims: Claims,
}

impl<'a, Claims> Jwt<'a, Claims> {
    pub fn new(secret: &'a str, alg: Algorithm, claims: Claims) -> Self {
        Jwt {
            secret,
            headers: JwtHeaders {
                alg,
                ..Default::default()
            },
            claims,
        }
    }

    fn new_hasher(secret: &str) -> HmacSha256 {
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size")
    }
}

impl<'a, Claims> Jwt<'a, Claims>
where
    Claims: Serialize + DeserializeOwned,
{
    pub fn encode(&self) -> Result<String, JwtError> {
        let start = Instant::now();

        let headers_ser = json!(self.headers).to_string();
        let claims_ser = json!(self.claims).to_string();

        debug!("{}.{}", headers_ser, claims_ser);

        let payload = format!(
            "{}.{}",
            base64url_encode(headers_ser.as_bytes(), false),
            base64url_encode(claims_ser.as_bytes(), false)
        );

        debug!("before hashing {:?}", start.elapsed());

        // Handle other hashing algorithms
        let mut hasher = Self::new_hasher(self.secret);

        hasher.update(payload.as_bytes());

        // `result` has type `CtOutput` which is a thin wrapper around array of
        // bytes for providing constant time equality check
        let result = hasher.finalize().into_bytes();

        debug!("after hashing {:?}", start.elapsed());

        debug!("result: {:?}", hex(&result.as_slice()));

        let hmac = base64url_encode(&result, false);

        Ok(format!("{}.{}", payload, hmac))
    }

    pub fn try_decode<'f>(secret: &'f str, encoded: &str) -> Result<Jwt<'f, Claims>, JwtError> {
        let (payload, _) = encoded
            .rsplit_once('.')
            .ok_or(JwtError::ParseJwtError(Backtrace::capture()))?;

        let mut hasher = Self::new_hasher(secret);

        hasher.update(payload.as_bytes());

        // `result` has type `CtOutput` which is a thin wrapper around array of
        // bytes for providing constant time equality check
        let result = hasher.finalize().into_bytes();

        let hmac = base64url_encode(&result, false);

        let valid: bool = format!("{}.{}", payload, hmac)
            .as_bytes()
            .ct_eq(encoded.as_bytes())
            .into();

        let jwt = valid
            .then_some(payload)
            .ok_or(JwtError::VerificationError)
            .map(|payload| -> Result<Jwt<'_, Claims>, JwtError> {
                let (headers_ser, claims_ser) = payload
                    .split_once('.')
                    .ok_or(JwtError::ParseJwtError(Backtrace::capture()))?;

                let headers = base64_decode(headers_ser);
                let headers = from_str(&headers).map_err(|_| JwtError::ParseHeadersError)?;

                let claims = base64_decode(claims_ser);
                let claims_ref: &'_ String = &claims;
                let claims = from_str(claims_ref).map_err(|_| JwtError::ParseClaimsError)?;

                Ok(Jwt {
                    secret,
                    headers,
                    claims,
                })
            })?;

        jwt
    }
}

#[cfg(test)]
mod test {
    // use std::assert_matches::assert_matches;

    use indexmap::IndexMap;
    use serde_json::Number;
    use serde_json::Value;

    use super::*;

    #[ignore]
    #[test]
    fn test_encode() {
        let jwt_expected = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.XbPfbIHMI6arZ3Y922BhjWgQzWXcXNrz0ogtVhfEd2o";

        let mut jwt = Jwt::<'_, IndexMap<String, Value>>::new(
            "secret",
            Default::default(),
            Default::default(),
        );

        jwt.claims
            .insert("sub".to_string(), Value::String("1234567890".to_string()));
        jwt.claims
            .insert("name".to_string(), Value::String("John Doe".to_string()));
        jwt.claims.insert(
            "iat".to_string(),
            Value::Number(Number::from(1516239022u64)),
        );

        assert_eq!(jwt.encode().unwrap(), jwt_expected);
    }

    #[test]
    fn it_should_decode() -> Result<(), JwtError> {
        let jwt_encoded = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.XbPfbIHMI6arZ3Y922BhjWgQzWXcXNrz0ogtVhfEd2o";

        let jwt = Jwt::<'_, IndexMap<String, Value>>::try_decode("secret", jwt_encoded)?;

        assert_eq!(jwt.claims.get("name").unwrap().as_str(), Some("John Doe"));

        Ok(())
    }

    #[ignore]
    #[test]
    fn it_should_fail_to_decode_when_wrong_format() {
        let maybe_jwt = Jwt::<'_, IndexMap<String, Value>>::try_decode("secret", "wrong");
        // https://github.com/rust-lang/rust/issues/82775
        // Replace with the following in the future
        // assert_matches!(err, JwtError::ParseJwtError(_));
        let JwtError::ParseJwtError(_) = maybe_jwt.expect_err("Must fail") else {
            panic!("Wrong error");
        };
    }

    #[ignore]
    #[test]
    fn it_should_fail_to_decode_when_invalid_hash() {
        let jwt_encoded = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.XbPfbIHMI6arZ3Y922BhjWgQzWXcXNrz0ogtvHACKED";

        let maybe_jwt = Jwt::<'_, IndexMap<String, Value>>::try_decode("secret", jwt_encoded);

        match maybe_jwt {
            Err(JwtError::VerificationError) => {}
            Err(_) => panic!("Wrong error"),
            _ => panic!("Must fail"),
        }
    }
}
