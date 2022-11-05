import { Plugin, FieldEvent, DriverStation, FieldState, AllianceStation, TournamentLevel, DriverStationEvent } from "../src";

async function main() {
    let plugin = new Plugin("HJgywCoI9e0MGiWEnzeRfaAl", {
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

    

    ds.on(DriverStationEvent.UPDATE, () => {
        console.log(ds.getConnection());
    })
}

main();