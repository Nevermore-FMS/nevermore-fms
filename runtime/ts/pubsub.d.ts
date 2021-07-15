declare namespace Nevermore {
    /**
     * The `Nevermore.PubSub` namespace defines the API used to send messages to and from the frontend.
     */
    namespace PubSub {
        export type PubSubMessageImpl = object | number | string | null

        /**
         * Message represents and JS object capable of being turned into JSON.
         */
        export type PubSubMessage = PubSubMessageImpl | PubSubMessageImpl[]

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