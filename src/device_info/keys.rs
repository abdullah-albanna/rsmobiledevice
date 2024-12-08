use std::fmt::{self, Display};

#[derive(Debug)]
pub enum DeviceKeys {
    ActivationState,
    ActivationStateAcknowledged,
    BasebandActivationTicketVersion,
    BasebandCertId,
    BasebandChipID,
    BasebandKeyHashInformation,
    BasebandMasterKeyHash,
    BasebandRegionSKU,
    BasebandSerialNumber,
    BasebandStatus,
    BasebandVersion,
    BluetoothAddress,
    BoardId,
    BootSessionID,
    BrickState,
    BuildVersion,
    CPUArchitecture,
    CarrierBundleInfoArray,
    CertID,
    ChipID,
    ChipSerialNo,
    DeviceClass,
    DeviceColor,
    DeviceName,
    DieID,
    EthernetAddress,
    FirmwareVersion,
    FusingStatus,
    HardwareModel,
    HardwarePlatform,
    HasSiDP,
    HostAttached,
    HumanReadableProductVersionString,
    IntegratedCircuitCardIdentity,
    InternationalMobileEquipmentIdentity,
    InternationalMobileEquipmentIdentity2,
    InternationalMobileSubscriberIdentity,
    InternationalMobileSubscriberIdentityOverride,
    MLBSerialNumber,
    MobileEquipmentIdentifier,
    MobileSubscriberCountryCode,
    MobileSubscriberNetworkCode,
    ModelNumber,
    NonVolatileRAM,
    SystemAudioVolumeSaved,
    AutoBoot,
    BacklightLevel,
    BacklightNits,
    BootArgs,
    FMAccountMasked,
    FMActivationLocked,
    FMSpkeys,
    FMSpstatus,
    Obliteration,
    USBcFwflasherResult,
    PRIVersionMajor,
    PRIVersionMinor,
    PRIVersionReleaseNo,
    PairRecordProtectionClass,
    PartitionType,
    PasswordProtected,
    PhoneNumber,
    PkHash,
    ProductName,
    ProductType,
    ProductVersion,
    ProductionSOC,
    ProtocolVersion,
    ProximitySensorCalibration,
    RegionInfo,
    SIM1IsEmbedded,
    SIMStatus,
    SIMTrayStatus,
    SerialNumber,
    SoftwareBehavior,
    SoftwareBundleVersion,
    SupportedDeviceFamilies,
    TelephonyCapability,
    TimeIntervalSince1970,
    TimeZone,
    TimeZoneOffsetFromUTC,
    TrustedHostAttached,
    UniqueChipID,
    UniqueDeviceID,
    UseRaptorCerts,
    Uses24HourClock,
    WiFiAddress,
    WirelessBoardSerialNumber,
    CTPostponementInfoPRIVersion,
    CTPostponementInfoServiceProvisioningState,
    CTPostponementStatus,
    All,
}

impl Display for DeviceKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = String::new();
        match self {
            DeviceKeys::All => text.push_str(""),
            DeviceKeys::ActivationState => text.push_str("ActivationState"),
            DeviceKeys::ActivationStateAcknowledged => text.push_str("ActivationStateAcknowledged"),
            DeviceKeys::BasebandActivationTicketVersion => {
                text.push_str("BasebandActivationTicketVersion")
            }
            DeviceKeys::BasebandCertId => text.push_str("BasebandCertId"),
            DeviceKeys::BasebandChipID => text.push_str("BasebandChipID"),
            DeviceKeys::BasebandKeyHashInformation => text.push_str("BasebandKeyHashInformation"),
            DeviceKeys::BasebandMasterKeyHash => text.push_str("BasebandMasterKeyHash"),
            DeviceKeys::BasebandRegionSKU => text.push_str("BasebandRegionSKU"),
            DeviceKeys::BasebandSerialNumber => text.push_str("BasebandSerialNumber"),
            DeviceKeys::BasebandStatus => text.push_str("BasebandStatus"),
            DeviceKeys::BasebandVersion => text.push_str("BasebandVersion"),
            DeviceKeys::BluetoothAddress => text.push_str("BluetoothAddress"),
            DeviceKeys::BoardId => text.push_str("BoardId"),
            DeviceKeys::BootSessionID => text.push_str("BootSessionID"),
            DeviceKeys::BrickState => text.push_str("BrickState"),
            DeviceKeys::BuildVersion => text.push_str("BuildVersion"),
            DeviceKeys::CPUArchitecture => text.push_str("CPUArchitecture"),
            DeviceKeys::CarrierBundleInfoArray => text.push_str("CarrierBundleInfoArray"),
            DeviceKeys::CertID => text.push_str("CertID"),
            DeviceKeys::ChipID => text.push_str("ChipID"),
            DeviceKeys::ChipSerialNo => text.push_str("ChipSerialNo"),
            DeviceKeys::DeviceClass => text.push_str("DeviceClass"),
            DeviceKeys::DeviceColor => text.push_str("DeviceColor"),
            DeviceKeys::DeviceName => text.push_str("DeviceName"),
            DeviceKeys::DieID => text.push_str("DieID"),
            DeviceKeys::EthernetAddress => text.push_str("EthernetAddress"),
            DeviceKeys::FirmwareVersion => text.push_str("FirmwareVersion"),
            DeviceKeys::FusingStatus => text.push_str("FusingStatus"),
            DeviceKeys::HardwareModel => text.push_str("HardwareModel"),
            DeviceKeys::HardwarePlatform => text.push_str("HardwarePlatform"),
            DeviceKeys::HasSiDP => text.push_str("HasSiDP"),
            DeviceKeys::HostAttached => text.push_str("HostAttached"),
            DeviceKeys::HumanReadableProductVersionString => {
                text.push_str("HumanReadableProductVersionString")
            }
            DeviceKeys::IntegratedCircuitCardIdentity => {
                text.push_str("IntegratedCircuitCardIdentity")
            }
            DeviceKeys::InternationalMobileEquipmentIdentity => {
                text.push_str("InternationalMobileEquipmentIdentity")
            }

            DeviceKeys::InternationalMobileEquipmentIdentity2 => {
                text.push_str("InternationalMobileEquipmentIdentity2")
            }

            DeviceKeys::InternationalMobileSubscriberIdentity => {
                text.push_str("InternationalMobileSubscriberIdentity")
            }
            DeviceKeys::InternationalMobileSubscriberIdentityOverride => {
                text.push_str("InternationalMobileSubscriberIdentityOverride")
            }
            DeviceKeys::MLBSerialNumber => text.push_str("MLBSerialNumber"),
            DeviceKeys::MobileEquipmentIdentifier => text.push_str("MobileEquipmentIdentifier"),
            DeviceKeys::MobileSubscriberCountryCode => text.push_str("MobileSubscriberCountryCode"),
            DeviceKeys::MobileSubscriberNetworkCode => text.push_str("MobileSubscriberNetworkCode"),
            DeviceKeys::ModelNumber => text.push_str("ModelNumber"),
            DeviceKeys::NonVolatileRAM => text.push_str("NonVolatileRAM"),
            DeviceKeys::SystemAudioVolumeSaved => text.push_str("SystemAudioVolumeSaved"),
            DeviceKeys::AutoBoot => text.push_str("AutoBoot"),
            DeviceKeys::BacklightLevel => text.push_str("BacklightLevel"),
            DeviceKeys::BacklightNits => text.push_str("BacklightNits"),
            DeviceKeys::BootArgs => text.push_str("BootArgs"),
            DeviceKeys::FMAccountMasked => text.push_str("FMAccountMasked"),
            DeviceKeys::FMActivationLocked => text.push_str("FMActivationLocked"),
            DeviceKeys::FMSpkeys => text.push_str("FMSpkeys"),
            DeviceKeys::FMSpstatus => text.push_str("FMSpstatus"),
            DeviceKeys::Obliteration => text.push_str("Obliteration"),
            DeviceKeys::USBcFwflasherResult => text.push_str("USBcFwflasherResult"),
            DeviceKeys::PRIVersionMajor => text.push_str("PRIVersionMajor"),
            DeviceKeys::PRIVersionMinor => text.push_str("PRIVersionMinor"),
            DeviceKeys::PRIVersionReleaseNo => text.push_str("PRIVersionReleaseNo"),
            DeviceKeys::PairRecordProtectionClass => text.push_str("PairRecordProtectionClass"),
            DeviceKeys::PartitionType => text.push_str("PartitionType"),
            DeviceKeys::PasswordProtected => text.push_str("PasswordProtected"),
            DeviceKeys::PhoneNumber => text.push_str("PhoneNumber"),
            DeviceKeys::PkHash => text.push_str("PkHash"),
            DeviceKeys::ProductName => text.push_str("ProductName"),
            DeviceKeys::ProductType => text.push_str("ProductType"),
            DeviceKeys::ProductVersion => text.push_str("ProductVersion"),
            DeviceKeys::ProductionSOC => text.push_str("ProductionSOC"),
            DeviceKeys::ProtocolVersion => text.push_str("ProtocolVersion"),
            DeviceKeys::ProximitySensorCalibration => text.push_str("ProximitySensorCalibration"),
            DeviceKeys::RegionInfo => text.push_str("RegionInfo"),
            DeviceKeys::SIM1IsEmbedded => text.push_str("SIM1IsEmbedded"),
            DeviceKeys::SIMStatus => text.push_str("SIMStatus"),
            DeviceKeys::SIMTrayStatus => text.push_str("SIMTrayStatus"),
            DeviceKeys::SerialNumber => text.push_str("SerialNumber"),
            DeviceKeys::SoftwareBehavior => text.push_str("SoftwareBehavior"),
            DeviceKeys::SoftwareBundleVersion => text.push_str("SoftwareBundleVersion"),
            DeviceKeys::SupportedDeviceFamilies => text.push_str("SupportedDeviceFamilies"),
            DeviceKeys::TelephonyCapability => text.push_str("TelephonyCapability"),
            DeviceKeys::TimeIntervalSince1970 => text.push_str("TimeIntervalSince1970"),
            DeviceKeys::TimeZone => text.push_str("TimeZone"),
            DeviceKeys::TimeZoneOffsetFromUTC => text.push_str("TimeZoneOffsetFromUTC"),
            DeviceKeys::TrustedHostAttached => text.push_str("TrustedHostAttached"),
            DeviceKeys::UniqueChipID => text.push_str("UniqueChipID"),
            DeviceKeys::UniqueDeviceID => text.push_str("UniqueDeviceID"),
            DeviceKeys::UseRaptorCerts => text.push_str("UseRaptorCerts"),
            DeviceKeys::Uses24HourClock => text.push_str("Uses24HourClock"),
            DeviceKeys::WiFiAddress => text.push_str("WiFiAddress"),
            DeviceKeys::WirelessBoardSerialNumber => text.push_str("WirelessBoardSerialNumber"),
            DeviceKeys::CTPostponementInfoPRIVersion => {
                text.push_str("CTPostponementInfoPRIVersion")
            }
            DeviceKeys::CTPostponementInfoServiceProvisioningState => {
                text.push_str("CTPostponementInfoServiceProvisioningState")
            }
            DeviceKeys::CTPostponementStatus => text.push_str("CTPostponementStatus"),
        }

        write!(f, "{}", text)
    }
}
