use std::fmt::Display;

#[derive(Debug)]
pub(crate) enum DevicePowerAction {
    Sleep,
    Shutdown(DiagnosticBehavior),
    Restart(DiagnosticBehavior),
}

#[derive(Debug)]
pub enum DiagnosticBehavior {
    /// wait until the diagnostic relay gets freed before execution
    WaitUntilDisconnected = 1 << 1, // Equivalent to 2
    ShowSuccessMessage = 1 << 2, // Equivalent to 4
    ShowFailureMessage = 1 << 3, // Equivalent to 8
}

pub enum IORegPlane {
    IODeviceTree,
    IOPower,
    IOService,
    None,
}

impl Display for IORegPlane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IORegPlane::None => write!(f, "None"),
            IORegPlane::IOPower => write!(f, "IOPower"),
            IORegPlane::IOService => write!(f, "IOService"),
            IORegPlane::IODeviceTree => write!(f, "IODeviceTree"),
        }
    }
}

pub enum DiagnosticType {
    All,
    WiFi,
    GasGauge,
    NAND,
}

impl Display for DiagnosticType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticType::All => write!(f, "All"),
            DiagnosticType::WiFi => write!(f, "WiFi"),
            DiagnosticType::NAND => write!(f, "NAND"),
            DiagnosticType::GasGauge => write!(f, "GasGauge"),
        }
    }
}
