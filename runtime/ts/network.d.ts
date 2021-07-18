declare namespace Nevermore {
    /** The `Nevermore.Network` namespace includes functions and types to register a network configurator to the FMS.
     * 
     * Javascript Example:
     * ```js
     * Nevermore.Network.registerConfigurator({
     *      name: "ubiquiti-edgerouter-x-full",
     *      readme: "# Ubqiuiti EdgeRouter Network Stack\nThis is the configuration used by the Field at AMRoC Tampa Bay.",
     *      author: "McMackety",
     *      url: "https://edgarallanohms.com",
     *      email: "macdonnell.chase@gmail.com",
     *      supportedHardware: ["Ubiquiti Controller", "EdgeRouter X"]
     *  }, {
     *      scan: async function(isFactory) {
     *          if (isFactory) {
     *              // Run Factory Scanning Code.
     *              return Nevermore.Network.SUCCESS
     *          } else {
     *              // Run Configured Scanning Code. 
     *              return Nevermore.Network.SUCCESS
     *          }
     *      },
     *      initialConfiguration: async function(password) {
     *          return Nevermore.Network.ERROR("Not implemented")
     *      },
     *      matchConfiguration: async function(allianceStationConfiguration) {
     *          return Nevermore.Network.ERROR("Not implemented")
     *      }
     *  });
     * ```
     */
    namespace Network {
        /**
         * Defines a successful callback from a configurator function.
         */
        export const SUCCESS: Error

        /**
         * Defines an Error/Success returned from a configurator function.
         */
        export type Error = string | null

        /**
         * Defines the info needed to register a configurator.
         * 
         * This info is displayed to end users to help them identify it.
         */
        export interface ConfiguratorInfo {
            name: string,
            readme: string,
            author: string,
            url: string,
            email: string,
            supportedHardware: string[],
            timeout: number
        }

        /**
         * The configuration for an AllianceStation on the Network.
         */
        export interface AllianceStationConfiguration {
            ssid: string,
            password: string
        }

        /**
         * The configurations for each AllianceStation on the Network.
         */
        export interface AllianceStationToConfiguration {
            red1: AllianceStationConfiguration,
            red2: AllianceStationConfiguration,
            red3: AllianceStationConfiguration,
            blue1: AllianceStationConfiguration,
            blue2: AllianceStationConfiguration,
            blue3: AllianceStationConfiguration
        }

        /**
         * The callbacks for a configurator.
         */
        export interface ConfiguratorCallbacks {
            scan: () => Promise<Error>,
            initialConfiguration: () => Promise<Error>,
            matchConfiguration: (allianceStationConfiguration: AllianceStationToConfiguration) => Promise<Error>
        }

        /**
         * Registers a new configurator with the FMS.
         * 
         * @param info The information defining the configurator.
         * @param callbacks The callbacks of the configurator.
         */
        export function registerConfigurator(info: ConfiguratorInfo, callbacks: ConfiguratorCallbacks)

        /**
         * Creates a new error for a callback.
         * 
         * @param message The message of the error.
         */
        export function ERROR(message: string): Error
    }
}