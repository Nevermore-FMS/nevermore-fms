import { Metadata } from '@grpc/grpc-js';
import DriverStation from './DriverStation';
import { Field } from './Field';
import { AllianceStation, DriverStationQueryType, PluginAPIClient, PluginMetadata } from './models/plugin';
const grpc = require('@grpc/grpc-js');

export default class Plugin {
  private rpcAddress: string;
  private rpcClient: PluginAPIClient;
  private meta: PluginMetadata;
  private registrationToken: string;
  private pluginToken: string = "";
  private field: Field = new Field(this);


  constructor(registrationToken: string, meta: PluginMetadata, rpcAddress: string = '10.0.100.5:5276') {
    this.rpcAddress = rpcAddress;
    this.rpcClient = new PluginAPIClient(this.rpcAddress, grpc.credentials.createInsecure());
    this.registrationToken = registrationToken;
    this.meta = meta;
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

  async getDriverstationByAllianceStation(station: AllianceStation): Promise<DriverStation | null> {
    
  }

  private getDriverstationByAllianceStationFromFMS(station: AllianceStation): Promise<DriverStation | null> {
    let promise = new Promise<any>((resolve, reject) => {
      this.rpcClient.getDriverStation({
        queryType: DriverStationQueryType.ALLIANCESTATION,
        teamNumber: 0,
        allianceStation: station
      },  this.generateMetadata(), (err, ds) => {
        if (err != null) {
          reject(err.message);
          return;
        }
        resolve(ds);
      })
    });
    return promise;
  }

  
  private getDriverstationByTeamNumberFromFMS(teamNumber: number): Promise<DriverStation | null> {
    let promise = new Promise<any>((resolve, reject) => {
      this.rpcClient.getDriverStation({
        queryType: DriverStationQueryType.TEAMNUMBER,
        teamNumber: teamNumber,
        allianceStation: AllianceStation.UNRECOGNIZED
      },  this.generateMetadata(), (err, ds) => {
        if (err != null) {
          reject(err.message);
          return;
        }
        resolve(ds);
      })
    });
    return promise;
  }
}
