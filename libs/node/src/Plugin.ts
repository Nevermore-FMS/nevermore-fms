import { Metadata } from '@grpc/grpc-js';
import DriverStation from './DriverStation';
import { Field } from './Field';
import { AllianceStation, DriverStation as RPCDriverStation, PluginAPIClient, PluginMetadata } from './models/plugin';
const grpc = require('@grpc/grpc-js');

export default class Plugin {
  private rpcAddress: string;
  private rpcClient: PluginAPIClient;
  private meta: PluginMetadata;
  private registrationToken: string;
  private pluginToken: string = "";
  private field: Field = new Field(this);
  private driverstations: DriverStation[] = [];


  constructor(registrationToken: string, meta: PluginMetadata, rpcAddress: string = '10.0.100.5:5276') {
    this.rpcAddress = rpcAddress;
    this.rpcClient = new PluginAPIClient(this.rpcAddress, grpc.credentials.createInsecure());
    this.registrationToken = registrationToken;
    this.meta = meta;
    this.forceUpdateDriverstations();
  }

  generateMetadata(): Metadata {
    let meta = new Metadata();
    meta.add("x-token", this.pluginToken);
    return meta;
  }

  getRpcClient(): PluginAPIClient {
    return this.rpcClient;
  }

  getField(): Field {
    return this.field;
  }

  generateControlID(name: string): string {
    return this.meta.id + ":" + name;
  }

  async registerWithFMS(): Promise<void> {
    let promise = new Promise<any>((_, reject) => {
      this.rpcClient.registerPlugin({
        registrationToken: this.registrationToken,
        plugin: this.meta
      },  this.generateMetadata(), (err, res) => {
        if (err != null) {
          reject(err.message);
          return;
        }
        this.pluginToken = res.token;
      })
    });
    return promise;
  }

  async getDriverstationByTeamNumber(teamNumber: number): Promise<DriverStation | null> {
    for (let ds of this.driverstations) {
      if (ds.getTeamNumber() == teamNumber) {
        return ds;
      }
    }
    return null;
  }

  async getDriverstationByAllianceStation(station: AllianceStation): Promise<DriverStation | null> {
    for (let ds of this.driverstations) {
      if (ds.getAllianceStation() == station) {
        return ds;
      }
    }
    return null;
  }

  private async forceUpdateDriverstations() {
    let driverstations = await this.getDriverStations();
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

  
  private getDriverStations(): Promise<RPCDriverStation[]> {
    let promise = new Promise<any>((resolve, reject) => {
      this.rpcClient.getDriverStations({},  this.generateMetadata(), (err, ds) => {
        if (err != null) {
          reject(err.message);
          return;
        }
        resolve(ds.driverStations);
      })
    });
    return promise;
  }

  private listenForUpdates() {
    let pluginThis = this;
    let listener = this.rpcClient.onDriverStationCreate({});
    listener.on("data", (rpcDS: RPCDriverStation) => {
      let out = pluginThis.driverstations.find((ds) => {
        return rpcDS.teamNumber == ds.getTeamNumber();
      });
      if (out != null) {
        out.update(rpcDS);
      } else {
        pluginThis.driverstations.push(new DriverStation(pluginThis, rpcDS))
      }
    });
    listener.on("end", () => {
      pluginThis.listenForUpdates();
    });
  }

  private listenForDeletions() {
    let pluginThis = this;
    let listener = this.rpcClient.onDriverStationDelete({});
    listener.on("data", (rpcDS: RPCDriverStation) => {
      pluginThis.driverstations = pluginThis.driverstations.filter((ds) => {
        return rpcDS.teamNumber != ds.getTeamNumber();
      });
    });
    listener.on("end", () => {
      pluginThis.listenForUpdates();
    });
  }
}
