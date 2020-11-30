/// <reference types="node" />
import { Agent } from 'http';
import { URL } from 'url';
import { Headers, HeadersInit } from 'node-fetch';
import { Request, RequestOptions } from './request';
import { Response } from './response';
export interface ClientOptions {
    agent?: Agent | ((parsedUrl: URL) => Agent);
    baseUrl?: string | URL;
    headers?: HeadersInit | Record<string, string | undefined>;
}
export declare class Client {
    agent?: Agent | ((parsedUrl: URL) => Agent);
    baseUrl: string | URL;
    headers: Headers;
    constructor(options?: ClientOptions);
    createRequest(info: string | URL | RequestOptions, init?: RequestOptions): Request;
    request(info: string | URL | RequestOptions, init?: RequestOptions): Promise<Response>;
    delete(info: string | URL | RequestOptions, init?: RequestOptions): Promise<Response>;
    get(info: string | URL | RequestOptions, init?: RequestOptions): Promise<Response>;
    head(info: string | URL | RequestOptions, init?: RequestOptions): Promise<Response>;
    options(info: string | URL | RequestOptions, init?: RequestOptions): Promise<Response>;
    patch(info: string | URL | RequestOptions, init?: RequestOptions): Promise<Response>;
    post(info: string | URL | RequestOptions, init?: RequestOptions): Promise<Response>;
    put(info: string | URL | RequestOptions, init?: RequestOptions): Promise<Response>;
}
