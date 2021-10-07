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
            pubSubMap[topic] = subscription;
            while (true) {
                try {
                    await callback(
                        JSON.parse(
                            await core.opAsync("op_subscription_next", subscription)
                        )
                    );
                } catch(_) {
                    return;
                }
            }
        },

        unsubscribe: async function (topic) {
            if (topic in pubSubMap) {
                await core.opAsync(
                    "op_unsubscribe",
                    pubSubMap[topic]
                );
                delete pubSubMap[topic];
            }
        },
    };
  })(this);
