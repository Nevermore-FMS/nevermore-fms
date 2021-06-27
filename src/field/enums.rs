use serde::{Serialize, Deserialize};

/// Represents the Alliance of a Robot. Whereby there is a Red and Blue side. (Hardcoded
/// due to it's use in the network protocol)
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum Alliance {
    Red = 0,
    Blue
}

impl Alliance {
    pub fn from_integer(integer: i32) -> Alliance {
        match integer {
            0 => Alliance::Red,
            1 => Alliance::Blue,
            _ => Alliance::Red
        }
    }

    pub fn to_integer(self) -> i32 {
        match self {
            Alliance::Red => 0,
            Alliance::Blue => 1,
        }
    }
}

/// Represents the AllianceStation of a Robot. There are six different alliance stations around
/// an FRC field, three on each side. (Hardcoded due to it's use in the network protocol)
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum AllianceStation {
    Red1 = 0,
    Red2,
    Red3,
    Blue1,
    Blue2,
    Blue3,
    None
}

impl AllianceStation {
    pub fn from_integer(integer: i32) -> AllianceStation {
        match integer {
            0 => AllianceStation::Red1,
            1 => AllianceStation::Red2,
            2 => AllianceStation::Red3,
            3 => AllianceStation::Blue1,
            4 => AllianceStation::Blue2,
            5 => AllianceStation::Blue3,
            _ => AllianceStation::Red1
        }
    }

    pub fn to_integer(self) -> i32 {
        match self {
            AllianceStation::Red1 => 0,
            AllianceStation::Red2 => 1,
            AllianceStation::Red3 => 2,
            AllianceStation::Blue1 => 3,
            AllianceStation::Blue2 => 4,
            AllianceStation::Blue3 => 5,
            AllianceStation::None => 0
        }
    }
}

/// Represents the Status of a Driverstation. Used to tell the operator of a Driverstation
/// whether they should be in a game and whether they're in the correct station. Send
/// `DriverstationStatus::Good` when in the correct position, `DriverstationStatus::Bad`
/// when in the wrong position, and `DriverstationStatus::Waiting` when the team isn't in
/// this match.
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum DriverstationStatus {
    Good = 0,
    Bad,
    Waiting
}

impl DriverstationStatus {
    pub fn from_integer(integer: i32) -> DriverstationStatus {
        match integer {
            0 => DriverstationStatus::Good,
            1 => DriverstationStatus::Bad,
            2 => DriverstationStatus::Waiting,
            _ => DriverstationStatus::Waiting,
        }
    }

    pub fn to_integer(self) -> i32 {
        match self {
            DriverstationStatus::Good => 0,
            DriverstationStatus::Bad => 1,
            DriverstationStatus::Waiting => 2,
        }
    }
}

/// Represents the Level of the current match being played. I don't really know why this is
/// sent to the Driverstations, but it's required so it must be included.
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum MatchLevel {
    Good = 0,
    Bad,
    Waiting
}

impl MatchLevel {
    pub fn from_integer(integer: i32) -> MatchLevel {
        match integer {
            0 => MatchLevel::Good,
            1 => MatchLevel::Bad,
            2 => MatchLevel::Waiting,
            _ => MatchLevel::Waiting,
        }
    }

    pub fn to_integer(self) -> i32 {
        match self {
            MatchLevel::Good => 0,
            MatchLevel::Bad => 1,
            MatchLevel::Waiting => 2,
        }
    }
}

/// Represents the Mode of a Robot. These values correspond to the values you can
/// get from WPILib and can set on the Driverstation when directly connected.
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum Mode {
    TeleOp = 0,
    Test,
    Autonomous,
}

impl Mode {
    pub fn from_integer(integer: i32) -> Mode {
        match integer {
            0 => Mode::TeleOp,
            1 => Mode::Test,
            2 => Mode::Autonomous,
            _ => Mode::TeleOp,
        }
    }

    pub fn to_integer(self) -> i32 {
        match self {
            Mode::TeleOp => 0,
            Mode::Test => 1,
            Mode::Autonomous => 2,
        }
    }
}