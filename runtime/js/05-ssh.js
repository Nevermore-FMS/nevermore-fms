((window) => {
    const Nevermore = window.__bootstrap.nevermore.Nevermore;
    const core = window.Deno.core;

    Nevermore.SSH = {
        connect: async function (addr) {
            let rid = await core.opAsync("op_create_ssh_client", {
                addr
            });
            return new Client(rid);
        }
    };

    class Client {
        constructor(rid) {
            this.rid = rid;
        }

        async authenticateWithPassword(username, password) {
            return await core.opAsync("op_authenticate_client_with_password", {
                session: this.rid,
                username,
                password
            });
        }

        async exec(wants_reply, command) {
            let resp = await core.opAsync("op_exec_client", {
                session: this.rid,
                wants_reply,
                command
            });

            console.log(resp)

            return {
                data: Uint8Array.from(resp.data),
                exitStatus: resp.exitStatus
            }
        }

        close() {
            core.drop(this.rid);
        }
    };

    Nevermore.SSH.Client = Client;
})(this);
