import { Request, Response } from 'detritus-rest';
import { Bucket } from './bucket';
import { Client } from './client';
export interface RestRequestOptions {
    errorOnRatelimit?: boolean;
    skipRatelimitCheck?: boolean;
}
export declare class RestRequest {
    readonly bucketPath?: string;
    _bucketHash?: string;
    _bucketKey?: string;
    client: Client;
    errorOnRatelimit?: boolean;
    maxRetries: number;
    request: Request;
    retries: number;
    retryDelay: number;
    skipRatelimitCheck?: boolean;
    constructor(client: Client, request: Request, options?: RestRequestOptions);
    get bucket(): Bucket | null;
    get bucketHash(): null | string;
    get bucketKey(): null | string;
    get shouldRatelimitCheck(): boolean;
    sendRequest(): Promise<Response>;
    send(): Promise<Response>;
}
