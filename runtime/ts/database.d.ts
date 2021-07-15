declare namespace Nevermore {
    /**
     * The `Nevermore.Database` namespace defines the functions involved with storing and retrieving persistent data using SQL.
     */
    namespace Database {
        /**
         * Retrieves the SQLDatabase assigned for this Plugin. The database is isolated based upon the name defined.
         * 
         * This function caches the result until `SQLDatabase.close` is called, so once a database is created it always will return the same database.
         * 
         * @param name The name of the database you want to retrieve.
         */
        export function get(name: string): Promise<SQLDatabase>

        export type ParameterImpl = object | number | string | null

        /**
         * Defines a JSON compatible parameter.
         */
        export type Parameter = ParameterImpl | ParameterImpl[]

        /**
         * An SQL Database backed by SQLite.
         */
        export class SQLDatabase {
            private constructor(rid: number)

            /**
             * Runs an SQL statement without returning anything.
             * 
             * @param stmt The SQL statement.
             * @param params The parameters for the statement.
             */
            run(stmt: string, params: Parameter[]): Promise<void>

            /**
             * Runs an SQL statement and returns the first row.
             * 
             * @param stmt The SQL statement.
             * @param params The parameters for the statement.
             */
            get<T>(stmt: string, params: Parameter[]): Promise<T>

            /**
             * Runs an SQL statement and returns all rows.
             * 
             * @param stmt The SQL statement.
             * @param params The parameters for the statement.
             */
            all<T>(stmt: string, params: Parameter[]): Promise<T[]>

            /**
             * Closes the SQLite database.
             */
            close(): void
        }
    }
}