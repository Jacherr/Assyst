/// <reference types="node" />
import { EventEmitter } from 'events';
declare let Emitter: typeof EventEmitter;
export declare class EventSpewer extends Emitter implements EventEmitter {
    /**
     * @ignore
     */
    _events: {
        [key: string]: any;
    } | undefined;
    /**
     * @ignore
     */
    _subscriptions: Array<EventSubscription> | undefined;
    constructor();
    hasEventListener(name: string | symbol): boolean;
    subscribe(name: string | symbol, listener: (...args: any[]) => void): EventSubscription;
    removeSubscription(subscription: EventSubscription): void;
    removeAllListeners(name?: string | symbol): this;
    removeAllSubscriptions(): this;
}
export declare class EventSubscription {
    listener: ((...args: any[]) => void) | null;
    name: string | symbol;
    spewer: EventSpewer | null;
    constructor(spewer: EventSpewer, name: string | symbol, listener: (...args: any[]) => void);
    remove(): void;
}
export {};
