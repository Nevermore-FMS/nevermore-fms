async function onTick() {
  console.log("Tick")
  let ds = await Nevermore.Field.getDriverStation(5276);
}

Nevermore.Field.on("tick", onTick)