((window) => {
    const Nevermore = window.__bootstrap.nevermore.Nevermore;
    const core = window.Deno.core;

    Nevermore.Network = {
        SUCCESS: null,
        registerConfigurator: async function(info, callbacks) {
            if (callbacks != null) {

                // TODO: Needs testing
                if (callbacks.scan != null && callbacks.initialConfiguration != null && callbacks.matchConfiguration != null) {
                    let rid = await core.opAsync("op_register_configurator", { info });
                    scanRunner(rid, callbacks.scan);
                    initialConfigurationRunner(rid, callbacks.initialConfiguration);
                    matchConfigurationRunner(rid, callbacks.matchConfiguration);
                    return
                }
                throw "Not all callbacks defined."
            }
            throw "callbacks is null."
        },
        ERROR: function(message) {
            return message;
        }
    };

    async function scanRunner(rid, scanCallback) {
        while (true) {
            await core.opAsync("op_next_scan", rid);
            let args = {
                id: rid,
                reply: await scanCallback()
            }
            await core.opAsync("op_reply_scan", args);
        }
    }

    async function initialConfigurationRunner(rid, initialConfigurationCallback) {
        while (true) {
            await core.opAsync("op_next_initial_configuration", rid);
            let args = {
                id: rid,
                reply: await initialConfigurationCallback()
            }
            await core.opAsync("op_reply_initial_configuration", args);
        }
    }

    async function matchConfigurationRunner(rid, matchConfigurationCallback) {
        while (true) {
            let map = await core.opAsync("op_next_match_configuration", rid);            
            let args = {
                id: rid,
                reply: await matchConfigurationCallback(map)
            }
            await core.opAsync("op_reply_match_configuration", args);
        }
    }
})(this);
