import { EventEmitter } from "eventemitter3";
import DriverStation from "./DriverStation";
import { AllianceStation, FieldConfiguration, FieldState, DriverStation as RPCDriverStation } from "./models/plugin";
import Plugin from "./Plugin";

enum FieldEvent {
    STATE_UPDATE = "state_update",
    TERMINATE = "terminate",
    DS_CREATE = "ds_create",
    DS_DELETE = "ds_delete"
}

/**
 * Represents the Field controlled by the FMS.
 * 
 * Refer to {@link Plugin} for grabbing the Field Object.
 * 
 * @alpha
 */
class Field extends EventEmitter<FieldEvent, any> {
    private plugin: Plugin;
    private fieldState: FieldState | null = null;
    private fieldConfig: FieldConfiguration | null = null;
    private emergencyStoppedTeams: number[] = [];
    private enabledTeams: number[] = [];
    private driverstations: DriverStation[] = [];

    constructor(plugin: Plugin) {
        super();
        this.plugin = plugin;
        this.forceUpdateDriverstations();
        this.forceUpdateFieldState();
        setInterval(() => {
            this.forceUpdateDriverstations();
            this.forceUpdateFieldState();
        }, 5000)
        this.listenForFieldUpdates();
        this.listenForFieldTermination();
        this.listenForDSUpdates();
        this.listenForDSDeletions();
    }

    // SETTER FUNCTIONS

    async setTeamNumbersEmergencyStopped(teamNumbers: number[]): Promise<void> {
        let promise = new Promise<void>((resolve, reject) => {
            this.plugin.getRpcClient().updateEstopper({ id: this.plugin.generateControlID("estop"), name: "Emergency Stop (NodeJS)", teamNumberEstopper: { estoppedTeamNumbers: teamNumbers } }, this.plugin.generateMetadata(), (err, _) => {
                if (err != null) {
                    throw err.message;
                }
                this.emergencyStoppedTeams = teamNumbers;
                resolve();
            })
        });
        return promise;
    }

    async setTeamNumbersEnabled(teamNumbers: number[]): Promise<void> {
        let promise = new Promise<void>((resolve, reject) => {
            this.plugin.getRpcClient().updateEnabler({ id: this.plugin.generateControlID("enable"), name: "Enable (NodeJS)", teamNumberEnabler: { approvedTeamNumbers: teamNumbers } }, this.plugin.generateMetadata(), (err, _) => {
                if (err != null) {
                    throw err.message;
                }
                this.enabledTeams = teamNumbers;
                resolve();
            })
        });
        return promise;
    }

    /**
     * Sets the FieldConfiguration of the Field. 
     * 
     * @param fieldConfig The FieldConfiguration containing Match Information.
     * @returns Nothing, but be sure to await this function.
     */
    async setFieldConfig(fieldConfig: FieldConfiguration): Promise<void> {
        let fieldThis = this;
        let promise = new Promise<void>((resolve, reject) => {
            this.plugin.getRpcClient().configureField(fieldConfig, this.plugin.generateMetadata(), (err, state) => {
                if (err != null) {
                    throw err.message;
                }
                fieldThis.fieldState = state;
                fieldThis.fieldConfig = fieldConfig;
                resolve();
            })
        });
        return promise;
    }

    /**
     * Sets the DiffTimer of the Field.
     * 
     * @param timeInMilliseconds The time in milliseconds you want to set the timer to.
     * @param autoCountdown Should the FMS automatically count down this timer?
     * @returns Nothing, but be sure to await this function.
     */
    async setTimer(timeInMilliseconds: number, autoCountdown: boolean): Promise<void> {
        let fieldThis = this;
        let promise = new Promise<void>((resolve, reject) => {
            this.plugin.getRpcClient().updateFieldTimer({ timeRemaining: timeInMilliseconds, running: autoCountdown }, this.plugin.generateMetadata(), (err, state) => {
                if (err != null) {
                    throw err.message;
                }
                fieldThis.fieldState = state;
                resolve();
            })
        });
        return promise;
    }

     async addDriverStation(teamNumber: number, allianceStation: AllianceStation): Promise<DriverStation> {
        let fieldThis = this;
        let promise = new Promise<any>((resolve, reject) => {
            this.plugin.getRpcClient().addDriverStation({ teamNumber, allianceStation }, this.plugin.generateMetadata(), (err, rpcDS) => {
                if (err != null) {
                    throw err.message;
                }
                let out = fieldThis.driverstations.find((ds) => {
                    return rpcDS.teamNumber == ds.getTeamNumber();
                });
                if (out != null) {
                    out.update(rpcDS);
                    resolve(out);
                } else {
                    let newDS = new DriverStation(fieldThis.plugin, rpcDS);
                    fieldThis.driverstations.push(newDS);
                    resolve(newDS);
                }
            })
        });
        return promise;
    }

    async removeDriverStation(teamNumber: number, allianceStation: AllianceStation): Promise<void> {
        let promise = new Promise<void>((resolve, reject) => {
            this.plugin.getRpcClient().deleteDriverStation({ teamNumber, allianceStation }, this.plugin.generateMetadata(), (err, _) => {
                if (err != null) {
                    throw err.message;
                }

                resolve();
            })
        });
        return promise;
    }

    // GETTER FUNCTIONS

    /**
     * Returns the current FieldState, as defined by the FMS.
     * 
     * @returns The current FieldState of the Field.
     */
    async getFieldState(): Promise<FieldState> {
        if (this.fieldState == null) {
            let state = await this.getFieldStateFromFMS();
            this.fieldState = state;
            return state;
        }
        return this.fieldState;
    }

    /**
     * Returns the current FieldConfig.
     * 
     * @returns The current FieldConfig of the Field.
     */
    getFieldConfig(): FieldConfiguration | null {
        return this.fieldConfig;
    }

    /**
     * Returns the Teams Emergency Stopped by the Plugin.
     * 
     * @returns List of Teams Numbers emergency stopped by Plugin.
     */
    getEmergencyStoppedTeams(): number[] {
        return this.emergencyStoppedTeams;
    }

    /**
     * Returns the Teams Enabled by the Plugin.
     * 
     * @returns List of Teams Numbers enabled by Plugin.
     */
    getEnabledTeams(): number[] {
        return this.enabledTeams;
    }

    // PRIVATE INTERNAL HELPER FUNCTIONS

    private listenForFieldUpdates() {
        let fieldThis = this;
        let listener = this.plugin.getRpcClient().onFieldStateUpdate({}, this.plugin.generateMetadata());
        listener.on("data", (state: FieldState) => {
            fieldThis.emit(FieldEvent.STATE_UPDATE, state);
            fieldThis.fieldState = state;
        });
        listener.on("end", () => {
            fieldThis.listenForFieldUpdates();
        });
    }

    private listenForFieldTermination() {
        let fieldThis = this;
        let listener = this.plugin.getRpcClient().onFieldTerminate({}, this.plugin.generateMetadata());
        listener.on("data", (state: FieldState) => {
            fieldThis.emit(FieldEvent.TERMINATE, state);
            fieldThis.fieldState = null;
        });
        listener.on("end", () => {
            fieldThis.listenForFieldTermination();
        });
    }

    private getFieldStateFromFMS(): Promise<FieldState> {
        let promise = new Promise<any>((resolve, reject) => {
            this.plugin.getRpcClient().getFieldState({
            }, this.plugin.generateMetadata(), (err, state) => {
                if (err != null) {
                    throw err.message;
                }
                resolve(state);
            })
        });
        return promise;
    }

    getDriverstationByTeamNumber(teamNumber: number): DriverStation | null {
        for (let ds of this.driverstations) {
            if (ds.getTeamNumber() == teamNumber) {
                return ds;
            }
        }
        return null;
    }

    getDriverstationByAllianceStation(station: AllianceStation): DriverStation | null {
        for (let ds of this.driverstations) {
            if (ds.getAllianceStation() == station) {
                return ds;
            }
        }
        return null;
    }

    getDriverstations(): DriverStation[] {
        return this.driverstations;
    }

    private async forceUpdateDriverstations() {
        let driverstations = await this.syncDriverStations();
        this.driverstations = this.driverstations.filter((testDS) => {
            let out = driverstations.find((ds) => {
                return testDS.getTeamNumber() == ds.teamNumber;
            });
            if (out != null) {
                testDS.update(out);
                return true;
            }
            return false;
        });
    }

    private async forceUpdateFieldState() {
        let state = await this.syncFieldState();
        this.fieldState = state;
    }


    private syncDriverStations(): Promise<RPCDriverStation[]> {
        let promise = new Promise<any>((resolve, reject) => {
            this.plugin.getRpcClient().getDriverStations({}, this.plugin.generateMetadata(), (err, ds) => {
                if (err != null) {
                    throw err.message;
                }
                resolve(ds.driverStations);
            })
        });
        return promise;
    }

    private syncFieldState(): Promise<FieldState> {
        let fieldThis = this;
        let promise = new Promise<any>((resolve, reject) => {
            this.plugin.getRpcClient().getFieldState({}, this.plugin.generateMetadata(), (err, state) => {
                if (err != null) {
                    throw err.message;
                }
                resolve(state);
            })
        });
        return promise;
    }

    private listenForDSUpdates() {
        let fieldThis = this;
        let listener = this.plugin.getRpcClient().onDriverStationCreate({}, this.plugin.generateMetadata());
        listener.on("data", (rpcDS: RPCDriverStation) => {
            let out = fieldThis.driverstations.find((ds) => {
                return rpcDS.teamNumber == ds.getTeamNumber();
            });
            if (out != null) {
                out.update(rpcDS);
                fieldThis.emit(FieldEvent.DS_CREATE, out);
            } else {
                let newDS = new DriverStation(fieldThis.plugin, rpcDS);
                fieldThis.driverstations.push(newDS);
                fieldThis.emit(FieldEvent.DS_CREATE, newDS);
            }
        });
        listener.on("end", () => {
            fieldThis.listenForDSUpdates();
        });
    }

    private listenForDSDeletions() {
        let fieldThis = this;
        let listener = this.plugin.getRpcClient().onDriverStationDelete({}, this.plugin.generateMetadata());
        listener.on("data", (rpcDS: RPCDriverStation) => {
            fieldThis.driverstations = fieldThis.driverstations.filter((ds) => {
                if (rpcDS.teamNumber == ds.getTeamNumber()) {
                    fieldThis.emit(FieldEvent.DS_DELETE, ds);
                }
                return rpcDS.teamNumber != ds.getTeamNumber();
            });
        });
        listener.on("end", () => {
            fieldThis.listenForDSDeletions();
        });
    }
}

export {
    Field,
    FieldEvent
}