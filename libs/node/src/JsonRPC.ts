import { JSONRPCServerAndClient, JSONRPCServer, JSONRPCClient } from "json-rpc-2.0";
import { JSONRpcMessage } from "./models/plugin";
import Plugin from "./Plugin";

interface ClientParams {
    pluginId: string
}

export default class JsonRPC {
    private serverClient: JSONRPCServerAndClient<void, ClientParams>;
    private plugin;

    constructor(plugin: Plugin) {
        this.plugin = plugin;
        this.serverClient = new JSONRPCServerAndClient(new JSONRPCServer(), new JSONRPCClient(async (request, clientParams: ClientParams | undefined) => {
            return new Promise<void>((resolve, reject) => {
                if (clientParams == undefined) {
                    reject("Invalid ClientParams");
                }
                let { pluginId } = clientParams as ClientParams;
                if (typeof pluginId != "string") {
                    reject("No PluginID included.")
                }
                this.plugin.getRpcClient().jsonRPCPublish({
                    pluginId,
                    data: JSON.stringify(request)
                }, this.plugin.generateMetadata(), (err, _) => {
                    if (err != null) {
                        reject(err);
                    }
                    resolve();
                })
            });
        }));
        this.listenForSubscriptions();
    }

    getJsonRPCServerAndClient(): JSONRPCServerAndClient<void, ClientParams> {
        return this.serverClient;
    }

    private listenForSubscriptions() {
        let rpcThis = this;
        let listener = this.plugin.getRpcClient().jsonRPCSubscribe({}, this.plugin.generateMetadata());
        listener.on("data", (msg: JSONRpcMessage) => {
            rpcThis.serverClient.receiveAndSend(JSON.parse(msg.data), undefined, { pluginId: msg.pluginId });
        });
        listener.on("end", () => {
            rpcThis.listenForSubscriptions();
        });
    }
}