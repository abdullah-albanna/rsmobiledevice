pub enum DeviceDomains {
    MobileDebug,
    MobileChaperone,
    MobileThirdPartyTermination,
    MobileBattery,
    MobileLockdownd,
    MobileLockdownCache,
    MobileDataSync,
    MobileTetheredSync,
    MobileMobileApplicationUsage,
    MobileBackup,
    MobileNikita,
    MobileRestriction,
    MobileUserPreferences,
    MobileSyncDataClass,
    MobileSoftwareBehavior,
    MobileITunesSQLMusicLibraryPostProcessCommands,
    MobileITunesAccessories,
    MobileITunesStore,
    MobileITunesITunes,
    MobileInternal,
    MobileWirelessLockdown,
    DiskUsage,
    DiskUsageFactory,
    Iqagent,
    FMIP,
    Accessibility,
    ITunes,
    Fairplay,
    PurpleBuddy,
    PurpleBuddy2,
    XCode,
    International,
    All,
}

impl DeviceDomains {
    pub fn as_string(&self) -> String {
        match self {
            DeviceDomains::All => "".into(),
            DeviceDomains::DiskUsage => "com.apple.disk_usage".into(),
            DeviceDomains::DiskUsageFactory => "com.apple.disk_usage.factory".into(),
            DeviceDomains::Iqagent => "".into(),
            DeviceDomains::FMIP => "com.apple.fmip".into(),
            DeviceDomains::Accessibility => "com.apple.Accessibility".into(),
            DeviceDomains::ITunes => "com.apple.iTunes".into(),
            DeviceDomains::Fairplay => "com.apple.fairplay".into(),
            DeviceDomains::PurpleBuddy => "com.apple.purplebuddy".into(),
            DeviceDomains::PurpleBuddy2 => "com.apple.PurpleBuddy".into(),
            DeviceDomains::XCode => "com.apple.xcode.developerdomain".into(),
            DeviceDomains::International => "com.apple.international".into(),
            DeviceDomains::MobileDebug => "com.apple.mobile.debug".into(),
            DeviceDomains::MobileChaperone => "com.apple.mobile.chaperone".into(),
            DeviceDomains::MobileThirdPartyTermination => {
                "com.apple.mobile.third_party_termination".into()
            }
            DeviceDomains::MobileBattery => "com.apple.mobile.battery".into(),
            DeviceDomains::MobileLockdownd => "com.apple.mobile.lockdownd".into(),
            DeviceDomains::MobileLockdownCache => "com.apple.mobile.lockdown_cache".into(),
            DeviceDomains::MobileDataSync => "com.apple.mobile.data_sync".into(),
            DeviceDomains::MobileTetheredSync => "com.apple.mobile.tethered_sync".into(),
            DeviceDomains::MobileMobileApplicationUsage => {
                "com.apple.mobile.mobile_application_usage".into()
            }
            DeviceDomains::MobileBackup => "com.apple.mobile.backup".into(),
            DeviceDomains::MobileNikita => "com.apple.mobile.nikita".into(),
            DeviceDomains::MobileRestriction => "com.apple.mobile.restriction".into(),
            DeviceDomains::MobileUserPreferences => "com.apple.mobile.user_preferences".into(),
            DeviceDomains::MobileSyncDataClass => "com.apple.mobile.sync_data_class".into(),
            DeviceDomains::MobileSoftwareBehavior => "com.apple.mobile.software_behavior".into(),
            DeviceDomains::MobileInternal => "com.apple.mobile.internal".into(),
            DeviceDomains::MobileWirelessLockdown => "com.apple.mobile.wireless_lockdown".into(),
            DeviceDomains::MobileITunesSQLMusicLibraryPostProcessCommands => {
                "com.apple.mobile.iTunes.SQLMusicLibraryPostProcessCommands".into()
            }
            DeviceDomains::MobileITunesAccessories => "com.apple.mobile.iTunes.accessories".into(),
            DeviceDomains::MobileITunesStore => "com.apple.mobile.iTunes.store".into(),
            DeviceDomains::MobileITunesITunes => "com.apple.mobile.iTunes".into(),
        }
    }
}
