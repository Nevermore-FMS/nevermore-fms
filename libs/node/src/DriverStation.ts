import {
    AllianceStation,
    DriverStation as DS,
    DriverStationConfirmedState,
    DriverStationConnection,
    Mode,
} from './models/plugin';
import Plugin from './Plugin';

/**
 * Represents a DriverStation that **could** be connected to the FMS.
 * 
 * Refer to {@link Plugin} for creating and delete DriverStations.
 * 
 * @alpha
 */
export default class DriverStation {
    private plugin: Plugin;
    private ds: DS;
    private mode: Mode = Mode.TELEOP;

    constructor(plugin: Plugin, ds: DS) {
        this.plugin = plugin;
        this.ds = ds;
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
        let promise = new Promise<any>((_, reject) => {
            this.plugin.getRpcClient().updateDriverStationExpectedIP({
                teamNumber: this.ds.teamNumber,
                expectedIp: cidr
            }, this.plugin.generateMetadata(), (err, ds) => {
                if (err != null) {
                    reject(err.message);
                    return;
                }
                driverstationSelf.ds = ds;
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
                    reject(err.message);
                    return;
                }
                driverstationSelf.mode = mode;
            })
        });
        return promise;
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
     * The expected CIDR of the DriverStation, or undefined if not defined.
     * 
     * If you are not familiar with Classless Inter-Domain Routing, be sure to read up on it:
     * https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing
     * 
     * @returns The expected CIDR of the DriverStation, or undefined if not defined. (Ex: "10.52.76/24")
     */
    getExpectedIP(): string | undefined {
        return this.ds.expectedIp;
    }
}
