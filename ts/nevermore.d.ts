/**
 * The `Nevermore` namespace defines the API used to interact with the Nevermore FMS from Worker Scripts.
 * 
 * Remember, playing with robots is dangerous, but controlling their safety system is even more dangerous.
 * Ensure you triple check, test, and verify your workers before ever using them in production.
 */
declare namespace Nevermore {

    /**
     * The `Nevermore.Field` namespace defines the API used to interact with the active field.
     * 
     * When the worker script is ran it's assumed that the current field has been fully started and 
     * all functions may be called.
     */
    namespace Field {

        export function on(event: 'tick', listener: () => Promise<void>): void;

        export function on(event: 'close', listener: () => Promise<void>): void;

        /**
         * Retrieves a teams DriverStation.
         * 
         * @param teamNumber The team number for the driver station.
         */
        export function getDriverStation(teamNumber: number): Promise<DriverStation>

        /**
         * Retrieves all connected DriverStations
         */
         export function getDriverStations(): Promise<DriverStation[]>

        /**
         * Adds a team to the alliance station map, thereby allowing it to properly connect.
         * 
         * @param teamNumber The team number of the team intended to be added.
         * @param allianceStation The alliance station of the team.
         */
        export function addTeam(teamNumber: number, allianceStation: AllianceStation): Promise<void>

        /**
         * Removes a team from the alliance station map, doesn't work after a robot is already connected.
         * 
         * @param teamNumber The team number of the team intended to be removed.
         */
        export function removeTeam(teamNumber: number): Promise<void>

        /**
         * Emergency stops all robots on the field no matter the state of any driverstations.
         * 
         * @param emergencyStopped Whether the robots should or shouldn't be emergency stopped.
         */
        export function setOverrideEmergencyStoppedAll(emergencyStopped: boolean): Promise<void>

        /**
         * Enables or disabes all robots on the field no matter the state of any driverstations.
         * 
         * @param enabled Whether the robots should or shouldn't be enabled.
         */
        export function setOverrideEnabledAll(enabled: boolean): Promise<void>

        export function getTeamAllianceStation(teamNumber: number): Promise<AllianceStation>

        export function getTeamToAllianceStationMap(): Promise<Map<number, AllianceStation>>

        export enum AllianceStation {
            RED1,
            RED2,
            RED3,
            BLUE1,
            BLUE2,
            BLUE3,
            NONE
        }

        export enum DriverStationStatus {
            GOOD,
            BAD,
            WAITING
        }

        export enum Mode {
            TELEOP,
            TEST,
            AUTONOMOUS
        }

        export interface DriverStationState {
            emergencyStop: boolean,
            enable: boolean,
            mode: Mode,
            teamNumber: number,
            allianceStation: AllianceStation,
            status: DriverStationStatus,
            sequenceNumber: number,
            timeToDisplay: number,
            matchNumber: number,
            eventName: string
        }

        export interface DriverStationConfirmedState {
            isEmergencyStopped: boolean,
            robotCommunicationsActive: boolean,
            canPingRadio: boolean,
            canPingRio: boolean,
            isEnabled: boolean,
            mode: Mode,
            teamNumber: number,
            batteryVoltage: number
        }

        export class DriverStation {
            private constructor(rid: number)

            getConfirmedState(): Promise<DriverStationConfirmedState>

            getState(): Promise<DriverStationState>

            setState(state: DriverStationState): Promise<DriverStationState>

            isInCorrectStation(): Promise<boolean>

            isInMatch(): Promise<boolean>

            getAddress(): Promise<string>

            isClosed(): Promise<boolean>
        }
    }

    /**
     * The `Nevermore.PubSub` namespace defines the API used to send messages to and from the frontend.
     */
    namespace PubSub {
        /**
         * Message represents and JS object capable of being turned into JSON.
         */
        export type PubSubMessage = any

        /**
         * A callback for a subscriber returning a JS Object.
         */
        export type PubSubCallback = (message: PubSubMessage) => Promise<void>;

        /**
         * Publishes the specified message to the specified topic.
         * 
         * Can be accessed on the frontend.
         * 
         * @param topic The topic being published to.
         * @param message The message to publish.
         */
        export function publish(topic: string, message: PubSubMessage): Promise<void>;

        /**
         * Subscribes to the specified topic and calls the callback when it receives a message.
         * 
         * @param topic The topic being subscribed to.
         * @param callback The callback to be called with the message as a param.
         */
        export function subscribe(topic: string, callback: PubSubCallback): void;

        /**
         * Unsubscribes from the specified topic and calls the callback when it receives a message.
         * 
         * @param topic The topic being unsubscribed from.
         * @param callback The callback being unsubscribed from.
         */
        export function unsubscribe(topic: string, callback: PubSubCallback): Promise<void>;
    }
}