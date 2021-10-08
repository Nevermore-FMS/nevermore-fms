import { TeamAllianceStation } from "../generated/graphql";

export function whoNotReady(expectedStations: TeamAllianceStation[], driverStations: Nevermore.Field.DriverStationConfirmedState[]): number[] {
    const notReady: number[] = []

    STATION:
    for (const exStation of expectedStations) {
        for (const ds of driverStations) {
            if (exStation.teamNumber === ds.teamNumber /*&& ds.canPingRadio && ds.canPingRio && ds.robotCommunicationsActive*/) {
                continue STATION;
            }
        }
        notReady.push(exStation.teamNumber)
    }

    return notReady
}