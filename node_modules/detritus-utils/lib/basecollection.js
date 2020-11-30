"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.BaseCollection = exports.BaseCollectionMixin = void 0;
const Timers = require("./timers");
class BaseCollectionMixin {
    get length() {
        return this.size;
    }
    get size() {
        return 0;
    }
    clear() {
    }
    entries() {
        return this[Symbol.iterator]();
    }
    every(func) {
        for (let [key, value] of this) {
            if (!func(value, key)) {
                return false;
            }
        }
        return true;
    }
    filter(func) {
        const map = [];
        for (let [key, value] of this) {
            if (func(value, key)) {
                map.push(value);
            }
        }
        return map;
    }
    find(func) {
        for (let [key, value] of this) {
            if (func(value, key)) {
                return value;
            }
        }
        return undefined;
    }
    first() {
        for (let [key, value] of this) {
            return value;
        }
    }
    forEach(func, thisArg) {
    }
    join(separator) {
        return this.toArray().join(separator);
    }
    map(func) {
        const map = [];
        for (let [key, value] of this) {
            map.push(func(value, key));
        }
        return map;
    }
    reduce(func, initialValue) {
        let reduced = initialValue;
        for (let [key, value] of this) {
            reduced = func(reduced, value);
        }
        return reduced;
    }
    some(func) {
        for (let [key, value] of this) {
            if (func(value, key)) {
                return true;
            }
        }
        return false;
    }
    sort(func) {
        return this.toArray().sort(func);
    }
    toArray() {
        return Array.from(this.values());
    }
    toJSON() {
        return this.toArray();
    }
    toString() {
        return this[Symbol.toStringTag];
    }
    *keys() {
    }
    *values() {
    }
    *[Symbol.iterator]() {
    }
    get [Symbol.toStringTag]() {
        return 'BaseCollection';
    }
}
exports.BaseCollectionMixin = BaseCollectionMixin;
class BaseCollection extends BaseCollectionMixin {
    constructor({ expire, intervalTime, limit } = {}) {
        super();
        this.cache = new Map();
        this.interval = undefined;
        this.intervalTime = 5000;
        this.limit = Infinity;
        if (expire !== undefined) {
            this.expire = expire;
        }
        if (intervalTime !== undefined) {
            this.intervalTime = intervalTime;
        }
        if (limit !== undefined) {
            this.limit = limit;
        }
        if (this.expire) {
            this.intervalTime = Math.min(this.expire, this.intervalTime);
        }
        Object.defineProperties(this, {
            _lastUsed: { enumerable: false, writable: true },
            cache: { enumerable: false },
            expire: { configurable: true, writable: false },
            interval: { enumerable: false, writeable: true },
            intervalTime: { configurable: true, enumerable: false, writable: false },
            limit: { enumerable: false },
        });
    }
    get lastUsed() {
        if (this._lastUsed) {
            return this._lastUsed;
        }
        if (this.expire) {
            return this._lastUsed = new Map();
        }
        return new Map();
    }
    get shouldStartInterval() {
        return !!this.intervalTime && (!this.interval || !!this.interval.hasStarted);
    }
    setExpire(value) {
        Object.defineProperty(this, 'expire', { value });
        if (value) {
            if (this.size) {
                this.startInterval();
            }
        }
        else {
            this.stopInterval();
        }
        return this;
    }
    setIntervalTimeout(value) {
        Object.defineProperty(this, 'intervalTime', { value });
        if (value) {
            if (this.size) {
                this.startInterval();
            }
        }
        else {
            this.stopInterval();
        }
        return this;
    }
    startInterval() {
        if (this.intervalTime && this.expire) {
            if (!this.interval) {
                this.interval = new Timers.Interval();
            }
            this.interval.start(this.intervalTime, () => {
                const expire = this.expire;
                if (expire) {
                    const now = Date.now();
                    for (let [key, value] of this.cache) {
                        const lastUsed = this.lastUsed.get(key);
                        if (lastUsed && expire < now - lastUsed) {
                            this.delete(key);
                        }
                    }
                }
                else {
                    this.stopInterval();
                }
            });
        }
        else {
            this.stopInterval();
        }
    }
    stopInterval() {
        if (this.interval) {
            this.interval.stop();
        }
        this.interval = undefined;
    }
    get size() {
        return this.cache.size;
    }
    clear() {
        this.stopInterval();
        this.cache.clear();
        if (this._lastUsed) {
            this._lastUsed.clear();
        }
    }
    clone() {
        const collection = new BaseCollection(this);
        for (let [key, value] of this) {
            collection.set(key, value);
        }
        return collection;
    }
    delete(key) {
        const deleted = this.cache.delete(key);
        if (this._lastUsed) {
            this._lastUsed.delete(key);
        }
        if (!this.cache.size) {
            this.stopInterval();
        }
        return deleted;
    }
    forEach(func, thisArg) {
        return this.cache.forEach(func, thisArg);
    }
    get(key) {
        const value = this.cache.get(key);
        if (this.expire && value) {
            this.lastUsed.set(key, Date.now());
        }
        return value;
    }
    has(key) {
        return this.cache.has(key);
    }
    keys() {
        return this.cache.keys();
    }
    set(key, value) {
        this.cache.set(key, value);
        if (this.expire) {
            this.lastUsed.set(key, Date.now());
            if (this.shouldStartInterval) {
                this.startInterval();
            }
        }
        if (this.limit !== Infinity) {
            if (this.limit <= this.cache.size) {
                for (let [key, value] of this.cache) {
                    this.delete(key);
                    break;
                }
            }
        }
        return this;
    }
    values() {
        return this.cache.values();
    }
    [Symbol.iterator]() {
        return this.cache[Symbol.iterator]();
    }
    get [Symbol.toStringTag]() {
        return `BaseCollection (${this.size.toLocaleString()} items)`;
    }
}
exports.BaseCollection = BaseCollection;
