import * as Timers from './timers';
export declare class BaseCollectionMixin<K, V> {
    get length(): number;
    get size(): number;
    clear(): void;
    entries(): IterableIterator<[K, V]>;
    every(func: (v: V, k: K) => boolean): boolean;
    filter(func: (v: V, k: K) => boolean): Array<V>;
    find(func: (v: V, k: K) => boolean): V | undefined;
    first(): V | undefined;
    forEach(func: (v: V, k: K, map: Map<K, V>) => void, thisArg?: any): void;
    join(separator?: string): string;
    map(func: (v: V, k: K) => any): Array<any>;
    reduce(func: (intial: any, v: V) => any, initialValue?: any): any;
    some(func: (v: V, k: K) => boolean): boolean;
    sort(func?: (x: V, y: V) => number): Array<V>;
    toArray(): Array<V>;
    toJSON(): Array<V>;
    toString(): string;
    keys(): IterableIterator<K>;
    values(): IterableIterator<V>;
    [Symbol.iterator](): IterableIterator<[K, V]>;
    get [Symbol.toStringTag](): string;
}
export interface BaseCollectionOptions {
    expire?: number;
    intervalTime?: number;
    limit?: number;
}
export declare class BaseCollection<K, V> extends BaseCollectionMixin<K, V> {
    readonly cache: Map<K, V>;
    _lastUsed?: Map<K, number>;
    expire?: number;
    interval?: Timers.Interval;
    intervalTime: number;
    limit: number;
    constructor({ expire, intervalTime, limit }?: BaseCollectionOptions);
    get lastUsed(): Map<K, number>;
    get shouldStartInterval(): boolean;
    setExpire(value: number): this;
    setIntervalTimeout(value: number): this;
    startInterval(): void;
    stopInterval(): void;
    get size(): number;
    clear(): void;
    clone(): BaseCollection<K, V>;
    delete(key: K): boolean;
    forEach(func: (v: V, k: K, map: Map<K, V>) => void, thisArg?: any): void;
    get(key: K): V | undefined;
    has(key: K): boolean;
    keys(): IterableIterator<K>;
    set(key: K, value: V): this;
    values(): IterableIterator<V>;
    [Symbol.iterator](): IterableIterator<[K, V]>;
    get [Symbol.toStringTag](): string;
}
