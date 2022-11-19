import { Metadata, MetadataValue } from '@grpc/grpc-js';
import  { DriverStation } from './DriverStation';
import { Field } from './Field';
import { AllianceStation, DriverStation as RPCDriverStation, PluginAPIClient, PluginMetadata } from './models/plugin';
import JsonRPC from './JsonRPC';
const grpc = require('@grpc/grpc-js');

export default class Plugin {
  private rpcAddress: string;
  private rpcClient: PluginAPIClient;
  private meta: PluginMetadata;
  private registrationToken: string;
  private pluginToken: string = "";
  private field: Field | null = null;
  private jsonRPC: JsonRPC | null = null;
  private heartbeatTimer: NodeJS.Timer | null = null;


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
    if (this.field == null) {
      throw "Plugin has not been registered yet";
    }
    return this.field;
  }

  getJsonRPC(): JsonRPC {
    if (this.jsonRPC == null) {
      throw "Plugin has not been registered yet";
    }
    return this.jsonRPC;
  }

  generateControlID(name: string): string {
    return this.meta.id + ":" + name;
  }

  async registerWithFMS(): Promise<void> {
    let promise = new Promise<any>((resolve, reject) => {
      this.rpcClient.registerPlugin({
        registrationToken: this.registrationToken,
        plugin: this.meta
      },  this.generateMetadata(), (err, res) => {
        if (err != null) {
          throw err.message;
        }
        this.pluginToken = res.token;
        this.field = new Field(this);
        this.jsonRPC = new JsonRPC(this);
        if (this.heartbeatTimer != null) {
          this.heartbeatTimer.unref();
        }
        this.heartbeatTimer = setInterval(() => {
          this.rpcClient.heartbeat({}, this.generateMetadata(), () => {});
        }, 500);
        resolve(null);
      })
    });
    return promise;
  }
}
