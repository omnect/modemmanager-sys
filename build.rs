use bindgen;
use bindgen::callbacks::{DeriveInfo, ParseCallbacks, TypeKind};
use std::{env, path::PathBuf};

const MODEM_MANAGER_PATH: &str = "MODEM_MANAGER_PATH";

#[derive(Debug)]
struct NumDerives {}

impl ParseCallbacks for NumDerives {
    fn add_derives(&self, info: &DeriveInfo<'_>) -> Vec<String> {
        if info.kind == TypeKind::Enum {
            vec!["FromPrimitive".to_string()]
        } else {
            vec![]
        }
    }
}

#[derive(Debug)]
struct ZbusDerives {}

impl ParseCallbacks for ZbusDerives {
    fn add_derives(&self, info: &DeriveInfo<'_>) -> Vec<String> {
        let mut derives = vec![
            "Deserialize".to_string(),
            "Serialize".to_string(),
            "Type".to_string(),
        ];

        // Value and OwnedValue don't support negative enum variants, these
        // conversions are implemented manually.
        if !(info.name == "MMModemState") && !(info.name == "MMOmaSessionState") {
            derives.extend_from_slice(&["Value".to_string(), "OwnedValue".to_string()]);
        }

        derives
    }
}

// create rust types from modem manager constants
fn create_modem_manager_types() {
    let include_paths = match env::var(MODEM_MANAGER_PATH) {
        Ok(path) => vec![PathBuf::from(format!("{}/include", path))],
        Err(_) => {
            let lib = pkg_config::Config::new().probe("ModemManager").unwrap();
            lib.include_paths
        }
    };

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=MODEM_MANAGER_PATH");

    let mut bindings_builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(
            include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy())),
        )
        .bitfield_enum("MMBearerAllowedAuth")
        .bitfield_enum("MMBearerApnType")
        .bitfield_enum("MMBearerIpFamily")
        .bitfield_enum("MMBearerRoamingAllowance")
        .bitfield_enum("MMModem3gppFacility")
        .bitfield_enum("MMModemAccessTechnology")
        .bitfield_enum("MMModemCapability")
        .bitfield_enum("MMModemFirmwareUpdateMethod")
        .bitfield_enum("MMModemLocationSource")
        .bitfield_enum("MMModemMode")
        .bitfield_enum("MMOmaFeature")
        .rustified_enum("MMBearerAccessTypePreference")
        .rustified_enum("MMBearerIpMethod")
        .rustified_enum("MMBearerMultiplexSupport")
        .rustified_enum("MMBearerProfileSource")
        .rustified_enum("MMBearerType")
        .rustified_enum("MMCallDirection")
        .rustified_enum("MMCallState")
        .rustified_enum("MMCallStateReason")
        .rustified_enum("MMCdmaActivationError")
        .rustified_enum("MMCellType")
        .rustified_enum("MMConnectionError")
        .rustified_enum("MMCoreError")
        .rustified_enum("MMFirmwareImageType")
        .rustified_enum("MMMessageError")
        .rustified_enum("MMMobileEquipmentError")
        .rustified_enum("MMModem3gppDrxCycle")
        .rustified_enum("MMModem3gppEpsUeModeOperation")
        .rustified_enum("MMModem3gppMicoMode")
        .rustified_enum("MMModem3gppNetworkAvailability")
        .rustified_enum("MMModem3gppPacketServiceState")
        .rustified_enum("MMModem3gppRegistrationState")
        .rustified_enum("MMModem3gppSubscriptionState")
        .rustified_enum("MMModem3gppUssdSessionState")
        .rustified_enum("MMModemBand")
        .rustified_enum("MMModemCdmaActivationState")
        .rustified_enum("MMModemCdmaRegistrationState")
        .rustified_enum("MMModemCdmaRmProtocol")
        .rustified_enum("MMModemContactsStorage")
        .rustified_enum("MMModemLocationAssistanceDataType")
        .rustified_enum("MMModemLock")
        .rustified_enum("MMModemPortType")
        .rustified_enum("MMModemPowerState")
        .rustified_enum("MMModemState")
        .rustified_enum("MMModemStateChangeReason")
        .rustified_enum("MMModemStateFailedReason")
        .rustified_enum("MMOmaSessionState")
        .rustified_enum("MMOmaSessionStateFailedReason")
        .rustified_enum("MMOmaSessionType")
        .rustified_enum("MMSerialError")
        .rustified_enum("MMSimEsimStatus")
        .rustified_enum("MMSimRemovability")
        .rustified_enum("MMSimType")
        .rustified_enum("MMSmsCdmaServiceCategory")
        .rustified_enum("MMSmsCdmaTeleserviceId")
        .rustified_enum("MMSmsDeliveryState")
        .rustified_enum("MMSmsPduType")
        .rustified_enum("MMSmsState")
        .rustified_enum("MMSmsStorage")
        .rustified_enum("MMSmsValidityType")
        .parse_callbacks(Box::new(NumDerives {}));

    if cfg!(feature = "zbus") {
        bindings_builder = bindings_builder.parse_callbacks(Box::new(ZbusDerives {}));
    }

    let bindings = bindings_builder
        .generate()
        .expect("unable to generate bindings");

    // write bindings to the $OUT_DIR/bindings.rs
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    create_modem_manager_types();
}
