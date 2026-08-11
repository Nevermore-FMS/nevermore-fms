use async_graphql::*;
use cidr::AnyIpCidr;

pub struct GQLIpCidr(pub AnyIpCidr);

#[Scalar(name = "IpCidr")]
impl ScalarType for GQLIpCidr {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            Ok(value.parse().map(GQLIpCidr)?)
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}