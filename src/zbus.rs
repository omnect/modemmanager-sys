use crate::{MMModemState, MMOmaSessionState};

use ::zbus::wire::{OwnedVariant, Variant};

use num::FromPrimitive;

impl TryFrom<OwnedVariant> for MMModemState {
    type Error = ::zbus::Error;

    fn try_from(value: OwnedVariant) -> Result<MMModemState, Self::Error> {
        TryFrom::<Variant<'_>>::try_from(value.into())
    }
}

impl TryFrom<Variant<'_>> for MMModemState {
    type Error = ::zbus::Error;

    fn try_from(value: Variant<'_>) -> Result<MMModemState, Self::Error> {
        let Variant::I32(num) = value else {
            return Err(::zbus::Error::IncorrectType);
        };

        FromPrimitive::from_i32(num).ok_or(::zbus::Error::IncorrectType)
    }
}

impl TryFrom<OwnedVariant> for MMOmaSessionState {
    type Error = ::zbus::Error;

    fn try_from(value: OwnedVariant) -> Result<MMOmaSessionState, Self::Error> {
        TryFrom::<Variant<'_>>::try_from(value.into())
    }
}

impl TryFrom<Variant<'_>> for MMOmaSessionState {
    type Error = ::zbus::Error;

    fn try_from(value: Variant<'_>) -> Result<MMOmaSessionState, Self::Error> {
        let Variant::I32(num) = value else {
            return Err(::zbus::Error::IncorrectType);
        };

        FromPrimitive::from_i32(num).ok_or(::zbus::Error::IncorrectType)
    }
}
