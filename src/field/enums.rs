use std::fmt;

// Represents the Mode of a DriverStation. These values correspond to the values you can
/// get from WPILib and can set on the Driverstation when directly connected.
#[derive(Clone, Copy)]
pub enum Mode {
    TeleOp,
    Test,
    Autonomous,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Test
    }
}

impl Mode {
    pub fn from_byte(integer: u8) -> Mode {
        match integer {
            0 => Mode::TeleOp,
            1 => Mode::Test,
            2 => Mode::Autonomous,
            _ => Mode::TeleOp,
        }
    }

    pub fn to_byte(self) -> u8 {
        match self {
            Mode::TeleOp => 0,
            Mode::Test => 1,
            Mode::Autonomous => 2,
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::TeleOp => write!(f, "TeleOp"),
            Mode::Test => write!(f, "Test"),
            Mode::Autonomous => write!(f, "Autonomous"),
        }
    }
}

/// Represents the Status of a Driverstation. Used to tell the operator of a Driverstation
/// whether they should be in a game and whether they're in the correct station. Send
/// `DriverstationStatus::Good` when in the correct position, `DriverstationStatus::Bad`
/// when in the wrong position, and `DriverstationStatus::Waiting` when the team isn't in
/// this match.
pub enum DriverstationStatus {
    Good,
    Bad,
    Waiting,
}

impl DriverstationStatus {
    pub fn from_byte(integer: u8) -> DriverstationStatus {
        match integer {
            0 => DriverstationStatus::Good,
            1 => DriverstationStatus::Bad,
            2 => DriverstationStatus::Waiting,
            _ => DriverstationStatus::Waiting,
        }
    }

    pub fn to_byte(self) -> u8 {
        match self {
            DriverstationStatus::Good => 0,
            DriverstationStatus::Bad => 1,
            DriverstationStatus::Waiting => 2,
        }
    }
}

impl fmt::Display for DriverstationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DriverstationStatus::Good => write!(f, "Good"),
            DriverstationStatus::Bad => write!(f, "Bad"),
            DriverstationStatus::Waiting => write!(f, "Waiting"),
        }
    }
}

/// Represents the AllianceStation of a DriverStation. There are six different alliance stations around
/// an FRC field, three on each side. (Hardcoded due to it's use in the network protocol)
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AllianceStation {
    Red1,
    Red2,
    Red3,
    Blue1,
    Blue2,
    Blue3,
    None,
}

impl AllianceStation {
    pub fn from_byte(integer: u8) -> AllianceStation {
        match integer {
            0 => AllianceStation::Red1,
            1 => AllianceStation::Red2,
            2 => AllianceStation::Red3,
            3 => AllianceStation::Blue1,
            4 => AllianceStation::Blue2,
            5 => AllianceStation::Blue3,
            _ => AllianceStation::Red1,
        }
    }

    pub fn to_byte(self) -> u8 {
        match self {
            AllianceStation::Red1 => 0,
            AllianceStation::Red2 => 1,
            AllianceStation::Red3 => 2,
            AllianceStation::Blue1 => 3,
            AllianceStation::Blue2 => 4,
            AllianceStation::Blue3 => 5,
            AllianceStation::None => 0,
        }
    }
}

impl fmt::Display for AllianceStation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AllianceStation::Red1 => write!(f, "Red1"),
            AllianceStation::Red2 => write!(f, "Red2"),
            AllianceStation::Red3 => write!(f, "Red3"),
            AllianceStation::Blue1 => write!(f, "Blue1"),
            AllianceStation::Blue2 => write!(f, "Blue2"),
            AllianceStation::Blue3 => write!(f, "Blue3"),
            AllianceStation::None => write!(f, "None"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TournamentLevel {
    Test,
    Practice,
    Qualification,
    Playoff,
}

impl TournamentLevel {
    pub fn from_byte(integer: u8) -> TournamentLevel {
        match integer {
            0 => TournamentLevel::Test,
            1 => TournamentLevel::Practice,
            2 => TournamentLevel::Qualification,
            3 => TournamentLevel::Playoff,
            _ => TournamentLevel::Test,
        }
    }

    pub fn to_byte(self) -> u8 {
        match self {
            TournamentLevel::Test => 0,
            TournamentLevel::Practice => 1,
            TournamentLevel::Qualification => 2,
            TournamentLevel::Playoff => 3,
        }
    }
}

impl fmt::Display for TournamentLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TournamentLevel::Playoff => write!(f, "Playoff"),
            TournamentLevel::Practice => write!(f, "Practice"),
            TournamentLevel::Qualification => write!(f, "Qualification"),
            TournamentLevel::Test => write!(f, "Test"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Version {
    pub status: String,
    pub version: String
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum VersionType {
    WPILib,
    RoboRIO,
    DS,
    PDP,
    PCM,
    CANJag,
    CANTalon,
    ThirdParty
}

impl VersionType {
    pub fn from_byte(integer: u8) -> VersionType {
        match integer {
            0x00 => VersionType::WPILib,
            0x01 => VersionType::RoboRIO,
            0x02 => VersionType::DS,
            0x03 => VersionType::PDP,
            0x04 => VersionType::PCM,
            0x05 => VersionType::CANJag,
            0x06 => VersionType::CANTalon,
            0x07 => VersionType::ThirdParty,
            _ => VersionType::WPILib
        }
    }
}

impl fmt::Display for VersionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersionType::WPILib => write!(f, "WPILib"),
            VersionType::RoboRIO => write!(f, "RoboRIO"),
            VersionType::DS => write!(f, "DS"),
            VersionType::PDP => write!(f, "PDP"),
            VersionType::PCM => write!(f, "PCM"),
            VersionType::CANJag => write!(f, "CANJag"),
            VersionType::CANTalon => write!(f, "CANTalon"),
            VersionType::ThirdParty => write!(f, "ThirdParty")
        }
    }
}