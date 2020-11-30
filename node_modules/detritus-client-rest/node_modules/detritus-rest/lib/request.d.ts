/// <reference types="node" />
import { Agent } from 'http';
import { URL } from 'url';
import * as FormData from 'form-data';
import { BodyInit, Headers, HeadersInit, Request as FetchRequest, RequestRedirect } from 'node-fetch';
import { AbortSignal } from 'node-fetch/externals';
import { Response } from './response';
import { Route } from './route';
export interface RequestFile extends FormData.AppendOptions {
    key?: string;
    value: any;
}
export interface RequestOptions {
    agent?: Agent | ((parsedUrl: URL) => Agent);
    body?: BodyInit | null | any;
    compress?: boolean;
    files?: Array<RequestFile>;
    follow?: number;
    headers?: HeadersInit | Record<string, string | undefined>;
    jsonify?: boolean;
    method?: string;
    multipart?: boolean;
    path?: string;
    query?: Record<string, any>;
    redirect?: RequestRedirect;
    route?: Route | {
        method?: string;
        params?: Record<string, any>;
        path?: string;
    } | null;
    signal?: AbortSignal | null;
    size?: number;
    timeout?: number;
    url?: string | URL;
}
export declare class Request extends FetchRequest {
    readonly route: Route | null;
    constructor(info: string | URL | RequestOptions, init?: RequestOptions);
    get parsedUrl(): URL;
    clone(): Request;
    send(): Promise<Response>;
    toString(): string;
}
export declare function appendQuery(url: URL, key: string, value: any): void;
export declare function createHeaders(old?: HeadersInit | Record<string, string | undefined>): Headers;
