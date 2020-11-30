import { SocketStates } from '../constants';
export declare namespace ClusterIPCTypes {
    interface IPCMessage {
        op: number;
        data: any;
        request: boolean;
        shard?: number;
    }
    interface Close {
        code: number;
        reason: string;
        shardId: number;
    }
    interface Eval {
        error?: {
            message: string;
            name: string;
            stack: string;
        };
        code: string;
        ignored?: boolean;
        nonce: string;
        result?: any;
        results?: Array<[any, boolean]>;
    }
    interface IdentifyRequest {
        shardId: number;
    }
    interface ShardState {
        shardId: number;
        state: SocketStates;
    }
}
