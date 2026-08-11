use std::net::IpAddr;

use async_graphql::*;

pub struct GQLIpAddr(pub IpAddr);

#[Scalar(name = "IpAddr")]
impl ScalarType for GQLIpAddr {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            Ok(value.parse().map(GQLIpAddr)?)
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}