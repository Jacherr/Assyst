"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.EventSubscription = exports.EventSpewer = void 0;
const events_1 = require("events");
let Emitter = events_1.EventEmitter;
try {
    Emitter = require('eventemitter3');
}
catch (e) { }
class EventSpewer extends Emitter {
    constructor() {
        super();
        /**
         * @ignore
         */
        this._subscriptions = undefined;
        Object.defineProperties(this, {
            _events: { enumerable: false },
            _eventsCount: { enumerable: false },
            _maxListeners: { enumerable: false },
            _subscriptions: { enumerable: false },
        });
    }
    hasEventListener(name) {
        return !!this._events && (name in this._events);
    }
    subscribe(name, listener) {
        const subscription = new EventSubscription(this, name, listener);
        this.on(name, listener);
        if (!this._subscriptions) {
            this._subscriptions = [];
        }
        this._subscriptions.push(subscription);
        return subscription;
    }
    removeSubscription(subscription) {
        if (subscription.listener) {
            this.removeListener(subscription.name, subscription.listener);
        }
        if (this._subscriptions) {
            const index = this._subscriptions.indexOf(subscription);
            if (index !== 1) {
                if (this._subscriptions.length === 1) {
                    this._subscriptions.pop();
                }
                else {
                    this._subscriptions.splice(index, 1);
                }
            }
            if (!this._subscriptions.length) {
                this._subscriptions = undefined;
            }
        }
    }
    removeAllListeners(name) {
        if (this._subscriptions) {
            if (name !== undefined) {
                for (let subscription of this._subscriptions) {
                    if (subscription.name === name) {
                        subscription.remove();
                    }
                }
            }
            else {
                while (this._subscriptions.length) {
                    const subscription = this._subscriptions.shift();
                    if (subscription) {
                        subscription.listener = null;
                        subscription.spewer = null;
                    }
                }
            }
        }
        return super.removeAllListeners(name);
    }
    removeAllSubscriptions() {
        if (this._subscriptions) {
            while (this._subscriptions.length) {
                const subscription = this._subscriptions.shift();
                if (subscription) {
                    subscription.remove();
                }
            }
        }
        return this;
    }
}
exports.EventSpewer = EventSpewer;
class EventSubscription {
    constructor(spewer, name, listener) {
        this.name = name;
        this.listener = listener;
        this.spewer = spewer;
    }
    remove() {
        if (this.spewer) {
            this.spewer.removeSubscription(this);
        }
        this.listener = null;
        this.spewer = null;
    }
}
exports.EventSubscription = EventSubscription;
