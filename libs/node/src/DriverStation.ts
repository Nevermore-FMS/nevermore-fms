import { EventEmitter } from 'eventemitter3';
import {
    AllianceStation,
    DriverStation as DS,
    DriverStationConfirmedState,
    DriverStationConnection,
    DriverStationQueryType,
    LogData,
    LogMessage,
    Mode,
    Version,
} from './models/plugin';
import Plugin from './Plugin';

enum DriverStationEvent {
    UPDATE = "update",
}

/**
 * Represents a DriverStation that **could** be connected to the FMS.
 * 
 * Refer to {@link Plugin} for creating and delete DriverStations.
 * 
 * @alpha
 */
class DriverStation extends EventEmitter<DriverStationEvent, void> {
    private plugin: Plugin;
    private ds: DS;
    private mode: Mode = Mode.TELEOP;

    constructor(plugin: Plugin, ds: DS) {
        super();
        this.plugin = plugin;
        this.ds = ds;
        this.listenForDriverStationUpdate()
    }

    // SETTER FUNCTIONS

    /**
     * Sets the CIDR IP the FMS expects the DriverStation to Connect with.
     * 
     * If you are not familiar with Classless Inter-Domain Routing, be sure to read up on it:
     * https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing
     * 
     * @param cidr The CIDR the FMS expects the DriverStation to Connect with.
     * @returns Nothing, but be sure to await this function.
     */
    async setExpectedIP(cidr: string): Promise<void> {
        let driverstationSelf = this;
        let promise = new Promise<any>((resolve, reject) => {
            this.plugin.getRpcClient().updateDriverStationExpectedIP({
                teamNumber: this.ds.teamNumber,
                expectedIp: cidr
            }, this.plugin.generateMetadata(), (err, ds) => {
                if (err != null) {
                    throw err.message;
                }
                driverstationSelf.ds = ds;
                resolve(null);
            })
        });
        return promise;
    }

    /**
     * Sets the current Mode of the DriverStation. (Ex: TEST or AUTONOMOUS)
     * 
     * @param mode The current Mode sent to the Driverstation.
     * @returns Nothing, but be sure to await this function.
     */
    async setMode(mode: Mode): Promise<void> {
        let driverstationSelf = this;
        let promise = new Promise<any>((_, reject) => {
            this.plugin.getRpcClient().updateDriverStationMode({
                teamNumber: this.ds.teamNumber,
                mode: mode
            }, this.plugin.generateMetadata(), (err, ds) => {
                if (err != null) {
                    throw err.message;
                }
                driverstationSelf.mode = mode;
            })
        });
        return promise;
    }

    async setEmergencyStop(emergencyStopped: boolean): Promise<void> {
        if (emergencyStopped) {
            let emergencyStoppedTeams = this.plugin.getField().getEmergencyStoppedTeams();
            if (!emergencyStoppedTeams.includes(this.ds.teamNumber)) {
                emergencyStoppedTeams.push(this.ds.teamNumber);
                await this.plugin.getField().setTeamNumbersEmergencyStopped(emergencyStoppedTeams);
            }
        } else {
            await this.plugin.getField().setTeamNumbersEmergencyStopped(this.plugin.getField().getEmergencyStoppedTeams().filter((teamNumber) => teamNumber != this.ds.teamNumber));
        }
    }

    async setEnabled(enabled: boolean): Promise<void> {
        if (enabled) {
            let enabledTeams = this.plugin.getField().getEnabledTeams();
            if (!enabledTeams.includes(this.ds.teamNumber)) {
                enabledTeams.push(this.ds.teamNumber);
                await this.plugin.getField().setTeamNumbersEnabled(enabledTeams);
            }
        } else {
            await this.plugin.getField().setTeamNumbersEnabled(this.plugin.getField().getEnabledTeams().filter((teamNumber) => teamNumber != this.ds.teamNumber));
        }
    }

    update(ds: DS) {
        this.ds = ds;
    }

    // GETTER FUNCTIONS

    /**
     * Returns the current Mode being sent to the Driverstation.
     * 
     * @returns The current Mode sent to the Driverstation. (Ex: TEST or AUTONOMOUS)
     */
    getMode(): Mode {
        return this.mode;
    }

    /**
     * Returns the current DriverStationConnection if a DriverStation has made connection to the FMS,
     * otherwise it returns undefined if this DriverStation has not connected to the FMS.
     * 
     * @returns The active DriverStationConnection if a connection has been made, or undefined if no connection to the Driverstation is available.
     */
    getConnection(): DriverStationConnection | undefined {
        return this.ds.connection;
    }

    /**
     * Returns the last DriverStationConfirmedState recieved by the FMS from the DriverStation,
     * or undefined if it has not recieved any DriverStationConfirmedState yet.
     * 
     * @returns The last ConfirmedState sent by the DriverStationConfirmedState, or undefined if no connection has been sent.
     */
    getConfirmedState(): DriverStationConfirmedState | undefined {
        return this.ds.confirmedState;
    }

    /**
     * Returns the team number of the DriverStation. Must be between 0 and 9999.
     * 
     * @returns The team number of the DriverStation. (Ex: 5276)
     */
    getTeamNumber(): number {
        return this.ds.teamNumber;
    }

    /**
     * Returns the AllianceStation this DriverStation should be in.
     * 
     * @returns The AllianceStation this DriverStation should be in. (Ex: RED1)
     */
    getAllianceStation(): AllianceStation {
        return this.ds.allianceStation;
    }

    /**
     * Returns the latest LogData for the DriverStation
     * 
     * @returns The latest LogData for the DS or undefined if none has been recieved yet.
     */
     getLogData(): LogData | undefined {
        return this.ds.logData;
    }

    /**
     * Returns the version data for the DriverStation
     * 
     * @returns An array of all version data gathered by the FMS.
     */
     getVersionData(): Version[] {
        return this.ds.versions;
    }

    /**
     * The expected CIDR of the DriverStation, or undefined if not defined.
     * 
     * If you are not familiar with Classless Inter-Domain Routing, be sure to read up on it:
     * https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing
     * 
     * @returns The expected CIDR of the DriverStation, or undefined if not defined. (Ex: "10.52.76/24" for FRC Team 5276)
     */
    getExpectedIP(): string | undefined {
        return this.ds.expectedIp;
    }

    /**
     * Request all logs for a specific DriverStation from the FMS.
     * 
     * @returns A list of LogMessage, contains all Logs recieved from the DriverStation
     */
    async getLogs(): Promise<LogMessage[]> {
        let promise = new Promise<any>((resolve, reject) => {
            this.plugin.getRpcClient().getDriverStationLogs({ queryType: DriverStationQueryType.TEAMNUMBER, teamNumber: this.ds.teamNumber, allianceStation: this.ds.allianceStation }, this.plugin.generateMetadata(), (err, msgs) => {
                if (err != null) {
                    throw err.message;
                }
                resolve(msgs.messages);
            })
        });
        return promise;
    }

    private listenForDriverStationUpdate() {
        let dsThis = this;
        let listener = this.plugin.getRpcClient().onDriverStationUpdate({ queryType: DriverStationQueryType.TEAMNUMBER, teamNumber: this.ds.teamNumber, allianceStation: this.ds.allianceStation}, this.plugin.generateMetadata());
        listener.on("data", (driverstation: DS) => {
            dsThis.ds = driverstation;
            dsThis.emit(DriverStationEvent.UPDATE);
        });
        listener.on("end", () => {
            dsThis.listenForDriverStationUpdate();
        });
    }
}

export {
    DriverStation,
    DriverStationEvent
}
