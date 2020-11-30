/// <reference types="node" />
import { Blob, Headers, Response as FetchResponse } from 'node-fetch';
import { Request } from './request';
export declare class Response {
    readonly fetchResponse: FetchResponse;
    readonly request: Request;
    readonly took: number;
    _body: Promise<Buffer> | Buffer | null;
    constructor(request: Request, response: FetchResponse, took?: number);
    get body(): NodeJS.ReadableStream | null;
    get bodyUsed(): boolean;
    get headers(): Headers;
    get ok(): boolean;
    get redirected(): boolean;
    get size(): number;
    get status(): number;
    get statusCode(): number;
    get statusText(): string;
    get url(): string;
    arrayBuffer(): Promise<ArrayBuffer>;
    blob(): Promise<Blob>;
    buffer(): Promise<Buffer>;
    json(): Promise<unknown>;
    text(): Promise<string>;
    clone(): Response;
    toString(): string;
}
