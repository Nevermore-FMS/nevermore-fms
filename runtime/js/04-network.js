((window) => {
    const Nevermore = window.__bootstrap.nevermore.Nevermore;

    Nevermore.Network = {
        SUCCESS: null,
        registerConfigurator: async function(info, callbacks) {
            if (callbacks != null) {
                if (callbacks.scan != null && callbacks.initialConfiguration != null && callbacks.matchConfiguration != null) {
                    let rid = await Deno.core.opAsync("op_register_configurator", { info });
                    scanRunner(rid, callbacks.scan);
                    initialConfigurationRunner(rid, callbacks.initialConfiguration);
                    matchConfigurationRunner(rid, callbacks.matchConfiguration);
                }
                throw "Not all callbacks defined."
            }
            throw "callbacks is null."
        },
        error: function(message) {
            return message;
        }
    };

    async function scanRunner(rid, scanCallback) {
        while (true) {
            let isFactory = Deno.core.opAsync("op_next_scan", rid);
            let args = {
                id: rid,
                reply: await scanCallback(isFactory)
            }
            await Deno.core.opAsync("op_reply_initial_configuration", args);
        }
    }

    async function initialConfigurationRunner(rid, initialConfigurationCallback) {
        while (true) {
            let password = Deno.core.opAsync("op_next_initial_configuration", rid);
            let args = {
                id: rid,
                reply: await initialConfigurationCallback(password)
            }
            await Deno.core.opAsync("op_reply_initial_configuration", args);
        }
    }

    async function matchConfigurationRunner(rid, matchConfigurationCallback) {
        while (true) {
            let reply = Deno.core.opAsync("op_next_match_configuration", rid);            
            let args = {
                id: rid,
                reply: await matchConfigurationCallback(reply.password, reply.map)
            }
            await Deno.core.opAsync("op_reply_match_configuration", args);
        }
    }
})(this);
