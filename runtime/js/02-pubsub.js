((window) => {
    const pubSubMap = {};
    const Nevermore = window.__bootstrap.nevermore.Nevermore;
    const core = window.Deno.core;

    Nevermore.PubSub = {
        publish: async function (topic, message) {
            await core.opAsync("op_publish", {
                topic,
                message: JSON.stringify(message),
            });
        },

        subscribe: async function (topic, callback) {
            const subscription = await core.opAsync("op_subscribe", topic);
            pubSubMap[[topic, callback]] = subscription;
            while (true) {
                await callback(
                    JSON.parse(
                        await core.opAsync("op_subscription_next", subscription)
                    )
                );
            }
        },

        unsubscribe: async function (topic, callback) {
            if ([topic, callback] in pubSubMap) {
                await core.opAsync(
                    "op_unsubscribe",
                    pubSubMap[[topic, callback]]
                );
            }
        },
    };
  })(this);
