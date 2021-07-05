((window) => {
  const pubSubMap = {};
  const dsMap = {};

  const Nevermore = {
    Field: {
      AllianceStation: {
        RED1: 0,
        RED2: 1,
        RED3: 2,
        BLUE1: 3,
        BLUE2: 4,
        BLUE3: 5,
        NONE: 6,
      },

      DriverStationStatus: {
        GOOD: 0,
        BAD: 1,
        WAITING: 2,
      },

      Mode: {
        TELEOP: 0,
        TEST: 1,
        AUTONOMOUS: 2,
      },

      on: async function (name, callback) {
        switch (name) {
          case "tick": {
            const ticker = await Deno.core.opAsync("op_tick_subscribe");
            while (true) {
              try {
                await Deno.core.opAsync("op_tick_subscription_next", ticker);
              } catch (_) {
                break;
              }
              await callback();
            }
            break;
          }
          case "close": {
            const closer = await Deno.core.opAsync("op_close_subscribe");
            while (true) {
              try {
                await Deno.core.opAsync("op_close_subscription_next", closer);
              } catch (_) {
                break;
              }
              await callback();
            }
            break;
          }
          default:
            break;
        }
      },

      getDriverStations: async function (teamNumber) {
        const teamNumbers = await Deno.core.opAsync(
          "op_get_driver_station_team_numbers"
        );
        let driverStations = [];
        for (let teamNumber of teamNumbers) {
          try {
            driverStations.push(await Nevermore.Field.getDriverStation(teamNumber))
          } catch(_) {}
        }
        return driverStations;
      },

      getDriverStation: async function (teamNumber) {
        if (teamNumber in dsMap) {
          try {
            if (!(await dsMap[teamNumber].isClosed())) {
              return dsMap[teamNumber];
            }
          } catch (_) {}
        }
        const rid = await Deno.core.opAsync(
          "op_get_driver_station",
          teamNumber
        );
        const ds = new DriverStation(rid);
        dsMap[teamNumber] = ds;
        return ds;
      },

      addTeam: async function (teamNumber, allianceStation) {
        await Deno.core.opAsync("op_add_team", {
          teamNumber,
          allianceStation,
        });
      },

      removeTeam: async function (teamNumber) {
        await Deno.core.opAsync("op_remove_team", teamNumber);
      },

      setOverrideEmergencyStoppedAll: async function (emergencyStopped) {
        await Deno.core.opAsync("op_set_emergency_stop_all", emergencyStopped);
      },

      setOverrideEnabledAll: async function (enabled) {
        await Deno.core.opAsync("op_set_enabled_all", enabled);
      },

      getTeamAllianceStation: async function (teamNumber) {
        return await Deno.core.opAsync("op_get_team", teamNumber);
      },

      getTeamToAllianceStationMap: async function () {
        return await Deno.core.opAsync("op_get_team_map");
      },
    },

    PubSub: {
      publish: async function (topic, message) {
        await Deno.core.opAsync("op_publish", {
          topic,
          message: JSON.stringify(message),
        });
      },

      subscribe: async function (topic, callback) {
        const subscription = await Deno.core.opAsync("op_subscribe", topic);
        pubSubMap[[topic, callback]] = subscription;
        while (true) {
          await callback(
            JSON.parse(
              await Deno.core.opAsync("op_subscription_next", subscription)
            )
          );
        }
      },

      unsubscribe: async function (topic, callback) {
        if ([topic, callback] in pubSubMap) {
          await Deno.core.opAsync(
            "op_unsubscribe",
            pubSubMap[[topic, callback]]
          );
        }
      },
    },
  };

  class DriverStation {
    constructor(rid) {
      this.rid = rid;
    }

    async getConfirmedState() {
      return await Deno.core.opAsync(
        "op_driverstation_get_confirmed_state",
        this.rid
      );
    }

    async getState() {
      return await Deno.core.opAsync(
        "op_driverstation_get_state",
        this.rid
      );
    }

    async setState(state) {
      return await Deno.core.opAsync(
        "op_driverstation_set_state",
        {
          rid: this.rid,
          state
        }
      );
    }

    async isInCorrectStation() {
      return await Deno.core.opAsync(
        "op_driverstation_is_in_correct_station",
        this.rid
      );
    }

    async isInMatch() {
      return await Deno.core.opAsync(
        "op_driverstation_is_in_match",
        this.rid
      );
    }

    async getAddress() {
      return await Deno.core.opAsync(
        "op_driverstation_get_address",
        this.rid
      );
    }

    async isClosed() {
      return await Deno.core.opAsync(
        "op_driverstation_has_closed",
        this.rid
      );
    }
  }

  Nevermore.Field.DriverStation = DriverStation
  window.__bootstrap.nevermore = { Nevermore };
})(this);
