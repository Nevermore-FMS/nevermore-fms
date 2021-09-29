declare namespace Nevermore {
    /**
     * The `Nevermore.SSH` namespace defines the functions involved with connecting to a remote SSH server.
     */
    namespace SSH {
        /**
         * Connects to a SSH server address, returning a client.
         * 
         * @param address The address to connect to with SSH.
         */
        export function connect(address: string): Promise<Client>

        interface ExecResponse {
            data: Uint8Array,
            exit_status: number
        }

        /**
         * An SSH Client
         */
        export class Client {
            private constructor(rid: number)

            /**
             * Authenticates the Client with a username and password.
             * 
             * @param username The username used for auth.
             * @param password The password used for auth.
             */
            authenticateWithPassword(username: string, password: string): Promise<void>

            /**
             * Executes a command on it's own SSH channel and returns stdout and the exit status.
             * 
             * 
             * @param wantsReply Should actually reply? Or immediately return.
             * @param command The actual command that is ran.
             */
            exec(wantsReply: boolean, command: string): Promise<ExecResponse>

            /**
             * Closes the SSH connection.
             */
            close(): void
        }
    }
}