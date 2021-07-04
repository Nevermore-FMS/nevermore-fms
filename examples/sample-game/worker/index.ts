async function onTick() {
  let driverStations = await Nevermore.Field.getAllDriverStations()
  for (let driverStation of driverStations) {
    console.log(driverStation.getState())
  }
}

Nevermore.Field.on("tick", onTick)