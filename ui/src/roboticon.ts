export interface GameState {
    gameType: GameType,
    timeLeft: number,
    enabled: boolean,
    eStopped: boolean,
    driverStationInfo: Nevermore.Field.DriverStationConfirmedState[]
}

export enum GameType {
    BASIC,
    DANCEPARTY,
    SOCCER,
    STUNBALL
}

export enum DanceMove {
    MOVEQUAD1,
    MOVEQUAD2,
    MOVEQUAD3,
    MOVEQUAD4,
    SPINCLOCKWISE,
    SPINCOUNTERCLOCKWISE,
    SHAKE,
    SLIDE
}

export function DanceMoveToString(dm?: DanceMove): string {
    if (dm === DanceMove.MOVEQUAD1) {
        return "Move to quadrant 1"
    }
    if (dm === DanceMove.MOVEQUAD2) {
        return "Move to quadrant 2"
    }
    if (dm === DanceMove.MOVEQUAD3) {
        return "Move to quadrant 3"
    }
    if (dm === DanceMove.MOVEQUAD4) {
        return "Move to quadrant 4"
    }
    if (dm === DanceMove.SPINCLOCKWISE) {
        return "Spin Clockwise"
    }
    if (dm === DanceMove.SPINCOUNTERCLOCKWISE) {
        return "Spin Counter-clockwise"
    }
    if (dm === DanceMove.SHAKE) {
        return "Shake"
    }
    if (dm === DanceMove.SLIDE) {
        return "Slide"
    }
    return ""
}

export interface StunnedAlliances {
    redIsDisabled: boolean,
    redIsInCooldown: boolean,
    blueIsDisabled: boolean,
    blueIsInCooldown: boolean
}

export enum StunType {
    LOWERGOAL = 2,
    UPPERGOAL = 4,
    INNERGOAL = 7
}

export enum Alliance {
    RED,
    BLUE
}