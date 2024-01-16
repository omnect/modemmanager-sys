#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use num_derive::FromPrimitive;

#[cfg(feature = "zbus")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "zbus")]
use zvariant::{OwnedValue, Type, Value};

#[cfg(feature = "zbus")]
pub mod zbus;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
