import { EventEmitter } from "eventemitter3";
import { FieldConfiguration, FieldState } from "./models/plugin";
import Plugin from "./Plugin";

enum FieldStateEvent {
    STATE_UPDATE = "state_update",
    TERMINATE = "terminate"
}

class Field extends EventEmitter<FieldStateEvent, FieldState> {
    private plugin: Plugin;
    private fieldState: FieldState | null = null;
    private fieldConfig: FieldConfiguration | null = null;

    constructor(plugin: Plugin) {
        super();
        this.plugin = plugin;
        this.listenForUpdates();
        this.listenForTermination();
    }

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

    async getFieldState(): Promise<FieldState | null> {
        if (this.fieldState == null) {
            let state = await this.getFieldStateFromFMS();
            this.fieldState = state;
            return state;
        }
        return this.fieldState;
    }

    configure(fieldConfig: FieldConfiguration): Promise<void> {
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

    private getFieldStateFromFMS(): Promise<FieldState | null> {
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