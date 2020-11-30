/// <reference types="node" />
import { ChildProcess } from 'child_process';
import { EventSpewer } from 'detritus-utils';
import { ClusterManager } from '../clustermanager';
import { BaseCollection } from '../collections/basecollection';
import { ClusterIPCTypes } from './ipctypes';
export interface ClusterProcessOptions {
    clusterId: number;
    env: {
        [key: string]: string | undefined;
    };
    shardCount: number;
    shardEnd: number;
    shardStart: number;
}
export interface ClusterProcessRunOptions {
    timeout?: number;
    wait?: boolean;
}
export declare class ClusterProcess extends EventSpewer {
    readonly _evalsWaiting: BaseCollection<string, {
        promise: Promise<any>;
        resolve: Function;
        reject: Function;
    }>;
    readonly _shardsWaiting: BaseCollection<number, {
        resolve: Function;
        reject: Function;
    }>;
    readonly clusterId: number;
    readonly manager: ClusterManager;
    env: {
        [key: string]: string | undefined;
    };
    process: ChildProcess | null;
    constructor(manager: ClusterManager, options: ClusterProcessOptions);
    get file(): string;
    onMessage(message: ClusterIPCTypes.IPCMessage | any): Promise<void>;
    onExit(code: number, signal: string): Promise<void>;
    eval(code: Function | string, nonce: string): Promise<[any, boolean] | null>;
    send(message: any): Promise<void>;
    sendIPC(op: number, data?: any, request?: boolean, shard?: number): Promise<void>;
    run(options?: ClusterProcessRunOptions): Promise<ChildProcess>;
    on(event: string | symbol, listener: (...args: any[]) => void): this;
    on(event: 'ready', listener: () => any): this;
    on(event: 'shardClose', listener: (data: ClusterIPCTypes.Close) => any): this;
    on(event: 'shardState', listener: (data: ClusterIPCTypes.ShardState) => any): this;
    on(event: 'warn', listener: (payload: {
        error: Error;
    }) => any): this;
}
