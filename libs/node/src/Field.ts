import { EventEmitter } from "eventemitter3";
import { FieldConfiguration, FieldState } from "./models/plugin";
import Plugin from "./Plugin";

enum FieldStateEvent {
    STATE_UPDATE = "state_update",
    TERMINATE = "terminate"
}

/**
 * Represents the Field controlled by the FMS.
 * 
 * Refer to {@link Plugin} for grabbing the Field Object.
 * 
 * @alpha
 */
class Field extends EventEmitter<FieldStateEvent, FieldState> {
    private plugin: Plugin;
    private fieldState: FieldState | null = null;
    private fieldConfig: FieldConfiguration | null = null;
    private emergencyStopAll: boolean = false;
    private enableAll: boolean = false;

    constructor(plugin: Plugin) {
        super();
        this.plugin = plugin;
        this.listenForUpdates();
        this.listenForTermination();
    }

    // SETTER FUNCTIONS

    async setEmergencyStopAll(emergencyStopped: boolean): Promise<void> {
        let promise = new Promise<void>((resolve, reject) => {
            this.plugin.getRpcClient().updateEstopper({ id: this.plugin.generateControlID("estop-all"), name: "Emergency Stop All", allEstopper: {active: emergencyStopped}}, this.plugin.generateMetadata(), (err, _) => {
                if (err != null) {
                    reject(err.message);
                    return;
                }
                this.emergencyStopAll = emergencyStopped;
                resolve();
            })
        });
        return promise;
    }

    async setEnabledAll(enabled: boolean): Promise<void> {
        let promise = new Promise<void>((resolve, reject) => {
            this.plugin.getRpcClient().updateEnabler({ id: this.plugin.generateControlID("enable-all"), name: "Emergency Stop All", allEnabler: {active: enabled}}, this.plugin.generateMetadata(), (err, _) => {
                if (err != null) {
                    reject(err.message);
                    return;
                }
                this.enableAll = enabled;
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
                    reject(err.message);
                    return;
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
                    reject(err.message);
                    return;
                }
                fieldThis.fieldState = state;
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
     * Returns if the Field is Emergency Stopped by the Plugin.
     * 
     * @returns Is the Field Emergency Stopped by the Plugin?
     */
    getEmergencyStopAll(): boolean {
        return this.emergencyStopAll;
    }

    /**
     * Returns if the Field is Enabled by the Plugin.
     * 
     * @returns Is the Field Enabled by the Plugin?
     */
    getEnabledAll(): boolean {
        return this.enableAll;
    }

    // PRIVATE INTERNAL HELPER FUNCTIONS

    private listenForUpdates() {
        let fieldThis = this;
        let listener = this.plugin.getRpcClient().onFieldStateUpdate({});
        listener.on("data", (state: FieldState) => {
            fieldThis.emit(FieldStateEvent.STATE_UPDATE, state);
            fieldThis.fieldState = state;
        });
        listener.on("end", () => {
            fieldThis.listenForUpdates();
        });
    }

    private listenForTermination() {
        let fieldThis = this;
        let listener = this.plugin.getRpcClient().onFieldTerminate({});
        listener.on("data", (state: FieldState) => {
            fieldThis.emit(FieldStateEvent.TERMINATE, state);
            fieldThis.fieldState = null;
        });
        listener.on("end", () => {
            fieldThis.listenForTermination();
        });
    }

    private getFieldStateFromFMS(): Promise<FieldState> {
        let promise = new Promise<any>((resolve, reject) => {
            this.plugin.getRpcClient().getFieldState({
            }, this.plugin.generateMetadata(), (err, state) => {
                if (err != null) {
                    reject(err.message);
                    return;
                }
                resolve(state);
            })
        });
        return promise;
    }
}

export {
    Field,
    FieldStateEvent
}