pub enum DeviceDomains {
    Mobile(MobileDomains),
    FMIP,
    Accessibility,
    ITunes,
    Fairplay,
    PurpleBuddy,
    PurpleBuddy2,
    XCode,
    International,
    Iqagent,
    All,
}

pub enum MobileDomains {
    Debug,
    Chaperone,
    ThirdPartyTermination,
    Lockdown,
    LockdownCache,
    DataSync,
    TetheredSync,
    MobileApplicationUsage,
    Backup,
    Nikita,
    Restriction,
    UserPreferences,
    SyncDataClass,
    SoftwareBehavior,
    ITunes(ITunesDomains),
    Internal,
    WirelessLockdown,
}

pub enum ITunesDomains {
    SQLMusicLibraryPostProcessCommands,
    Accessories,
    Store,
    ITunes,
}

impl DeviceDomains {
    pub fn as_string(&self) -> String {
        match self {
            DeviceDomains::All => "".to_string(),
            DeviceDomains::Mobile(domain) => domain.as_string(),
            DeviceDomains::FMIP => "com.apple.fmip".to_string(),
            DeviceDomains::Accessibility => "com.apple.Accessibility".to_string(),
            DeviceDomains::ITunes => "com.apple.iTunes".to_string(),
            DeviceDomains::Fairplay => "com.apple.fairplay".to_string(),
            DeviceDomains::PurpleBuddy => "com.apple.purplebuddy".to_string(),
            DeviceDomains::PurpleBuddy2 => "com.apple.PurpleBuddy".to_string(),
            DeviceDomains::XCode => "com.apple.xcode.developerdomain".to_string(),
            DeviceDomains::International => "com.apple.international".to_string(),
            DeviceDomains::Iqagent => "com.apple.iqagent".to_string(),
        }
    }
}

impl MobileDomains {
    pub fn as_string(&self) -> String {
        match self {
            MobileDomains::Debug => "com.apple.mobile.debug".to_string(),
            MobileDomains::Chaperone => "com.apple.mobile.chaperone".to_string(),
            MobileDomains::ThirdPartyTermination => {
                "com.apple.mobile.third_party_termination".to_string()
            }
            MobileDomains::Lockdown => "com.apple.mobile.lockdownd".to_string(),
            MobileDomains::LockdownCache => "com.apple.mobile.lockdown_cache".to_string(),
            MobileDomains::DataSync => "com.apple.mobile.data_sync".to_string(),
            MobileDomains::TetheredSync => "com.apple.mobile.tethered_sync".to_string(),
            MobileDomains::MobileApplicationUsage => {
                "com.apple.mobile.mobile_application_usage".to_string()
            }
            MobileDomains::Backup => "com.apple.mobile.backup".to_string(),
            MobileDomains::Nikita => "com.apple.mobile.nikita".to_string(),
            MobileDomains::Restriction => "com.apple.mobile.restriction".to_string(),
            MobileDomains::UserPreferences => "com.apple.mobile.user_preferences".to_string(),
            MobileDomains::SyncDataClass => "com.apple.mobile.sync_data_class".to_string(),
            MobileDomains::SoftwareBehavior => "com.apple.mobile.software_behavior".to_string(),
            MobileDomains::ITunes(domain) => domain.as_string(),
            MobileDomains::Internal => "com.apple.mobile.internal".to_string(),
            MobileDomains::WirelessLockdown => "com.apple.mobile.wireless_lockdown".to_string(),
        }
    }
}

impl ITunesDomains {
    pub fn as_string(&self) -> String {
        match self {
            ITunesDomains::SQLMusicLibraryPostProcessCommands => {
                "com.apple.mobile.iTunes.SQLMusicLibraryPostProcessCommands".to_string()
            }
            ITunesDomains::Accessories => "com.apple.mobile.iTunes.accessories".to_string(),
            ITunesDomains::Store => "com.apple.mobile.iTunes.store".to_string(),
            ITunesDomains::ITunes => "com.apple.mobile.iTunes".to_string(),
        }
    }
}
