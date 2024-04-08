use crate::{MMModemState, MMOmaSessionState};

use zvariant::{OwnedValue, Value};

use num::FromPrimitive;

impl TryFrom<OwnedValue> for MMModemState {
    type Error = zvariant::Error;

    fn try_from(value: OwnedValue) -> Result<MMModemState, Self::Error> {
        TryFrom::<Value<'_>>::try_from(value.into())
    }
}

impl TryFrom<Value<'_>> for MMModemState {
    type Error = zvariant::Error;

    fn try_from(value: Value<'_>) -> Result<MMModemState, Self::Error> {
        let Value::I32(num) = value else {
            return Err(zvariant::Error::IncorrectType);
        };

        FromPrimitive::from_i32(num).ok_or(zvariant::Error::IncorrectType)
    }
}

impl TryFrom<OwnedValue> for MMOmaSessionState {
    type Error = zvariant::Error;

    fn try_from(value: OwnedValue) -> Result<MMOmaSessionState, Self::Error> {
        TryFrom::<Value<'_>>::try_from(value.into())
    }
}

impl TryFrom<Value<'_>> for MMOmaSessionState {
    type Error = zvariant::Error;

    fn try_from(value: Value<'_>) -> Result<MMOmaSessionState, Self::Error> {
        let Value::I32(num) = value else {
            return Err(zvariant::Error::IncorrectType);
        };

        FromPrimitive::from_i32(num).ok_or(zvariant::Error::IncorrectType)
    }
}
