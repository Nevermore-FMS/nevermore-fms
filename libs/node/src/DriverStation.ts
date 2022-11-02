import {
    AllianceStation,
    DriverStation as DS,
    DriverStationConfirmedState,
    DriverStationConnection,
    Mode,
} from './models/plugin';
import Plugin from './Plugin';

export default class DriverStation {
    private plugin: Plugin;
    private ds: DS;
    private mode: Mode = Mode.TELEOP;

    private constructor(plugin: Plugin, ds: DS) {
        this.plugin = plugin;
        this.ds = ds;
    }

    getTeamNumber(): number {
        return this.ds.teamNumber;
    }

    getAllianceStation(): AllianceStation {
        return this.ds.allianceStation;
    }

    getExpectedIP(): string | undefined {
        return this.ds.expectedIp;
    }

    setExpectedIP(cidr: string): Promise<void> {
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

    setMode(mode: Mode): Promise<void> {
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

    getMode(): Mode {
        return this.mode;
    }

    getConnection(): DriverStationConnection | undefined {
        return this.ds.connection;
    }

    getConfirmedState(): DriverStationConfirmedState | undefined {
        return this.ds.confirmedState;
    }
}
