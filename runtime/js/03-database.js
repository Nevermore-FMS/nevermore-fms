((window) => {
    const Nevermore = window.__bootstrap.nevermore.Nevermore;
    const core = window.Deno.core;
    let dbMap = {};

    Nevermore.Database = {
        get: async function (name) {
            if (name in dbMap) {
                return dbMap[name];
            }
            let db = new SQLDatabase(await core.opAsync("op_create_database", name));
            dbMap[name] = db;
            return db;
        }
    };

    class SQLDatabase {
        constructor(rid) {
            this.rid = rid;
        }

        async run(stmt, params) {
            return await core.opAsync("op_database_run", {
                stmt,
                params
            });
        }

        async get(stmt, params) {
            return await core.opAsync("op_database_get", {
                stmt,
                params
            });
        }

        async all(stmt, params) {
            return await core.opAsync("op_database_all", {
                stmt,
                params
            });
        }
    };

    Nevermore.Database.SQLDatabase = SQLDatabase;
})(this);
