import { Plugin, FieldEvent, DriverStation, FieldState, AllianceStation, TournamentLevel } from "../src";

async function main() {
    let plugin = new Plugin("rZMuJ5aDlCCNbQh5yUq0ADc5", {
        id: "test-plugin",
        name: "Test Plugin",
        authors: ["Chase MacDonnell"]
    }, "127.0.0.1:5276");

    console.log("Started Plugin...");

    try {
        await plugin.registerWithFMS();
    } catch (e) {
        return;
    }

    console.log(plugin.generateMetadata());

    console.log("Registered!");

    await plugin.getField().removeDriverStation(5276, AllianceStation.RED1);

    plugin.getField().on(FieldEvent.DS_CREATE, (state: DriverStation) => {
        console.log("Create: " + state.getTeamNumber());
    });

    plugin.getField().on(FieldEvent.DS_DELETE, (state: DriverStation) => {
        console.log("Delete: " + state.getTeamNumber());
    });

    plugin.getField().on(FieldEvent.STATE_UPDATE, (state: FieldState) => {
        console.log("State: " + state.eventName);
    });

    let ds = await plugin.getField().addDriverStation(5276, AllianceStation.RED1);
    await ds.setExpectedIP("10.0.100/24");

    setTimeout(async () => {
        await plugin.getField().setFieldConfig({
            eventName: "ROBOTICON",
            tournamentLevel: TournamentLevel.QUALIFICATION,
            matchNumber: 1,
            playNumber: 1
        })
    }, 5000);

    plugin.getJsonRPC().getJsonRPCServerAndClient().addMethod("test", ({ message }: any) => {
        return "Hello " + message;
    });

    for (let i = 0; i < 5; i++) {
        plugin.getJsonRPC().getJsonRPCServerAndClient().request("test", { message: "World!" }, { pluginId: "test-plugin" }).then((res) => {
            console.log(res);
        });
    }
}

main();