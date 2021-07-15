((window) => {
    const pubSubMap = {};
    const Nevermore = window.__bootstrap.nevermore;

    Nevermore.PubSub = {
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
    };
  })(this);
