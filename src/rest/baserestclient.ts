import { RatelimitInfo, RatelimitHeaders } from './types';

import fetch from 'node-fetch';
import { HttpMethods } from 'fapi-client/JS/src/types';

export class BaseRestClient {
    public baseUrl: string;
    public ratelimits?: RatelimitInfo
    public timeout: number = 20000;

    constructor(baseUrl: string) {
        this.baseUrl = baseUrl;
    }

    public async get<T = any>(endpoint: string): Promise<T> {
        const res: Response = await Promise.race([
            fetch(this.baseUrl + endpoint),
            new Promise((resolve, reject) => {
                setTimeout(() => reject(new Error(`Timeout after ${this.timeout}ms`)), this.timeout);
            })
        ]) as Response;

        const headers = res.headers;
        this.ratelimits = {
            [RatelimitHeaders.LIMIT]: parseInt(headers.get(RatelimitHeaders.LIMIT) as string),
            [RatelimitHeaders.REMAINING]: parseInt(headers.get(RatelimitHeaders.REMAINING) as string),
            [RatelimitHeaders.RESET]: parseInt(headers.get(RatelimitHeaders.RESET) as string)
        };

        if (!res.ok) throw new Error(await res.text() || res.statusText);
        const json = await res.json();
        return json;
    }

    public async post(endpoint: string, requestHeaders: { [key: string]: string }, body: { [key: string]: string | number | boolean }) {
        const res: Response = await Promise.race([
            fetch(this.baseUrl + endpoint, {
                method: HttpMethods.POST,
                body: JSON.stringify(body),
                headers: {
                    'content-type': 'application/json',
                    ...requestHeaders
                }
            }),
            new Promise((resolve, reject) => {
                setTimeout(() => reject(new Error(`Timeout after ${this.timeout}ms`)), this.timeout);
            })
        ]) as Response;

        const headers = res.headers;
        this.ratelimits = {
            [RatelimitHeaders.LIMIT]: parseInt(headers.get(RatelimitHeaders.LIMIT) as string),
            [RatelimitHeaders.REMAINING]: parseInt(headers.get(RatelimitHeaders.REMAINING) as string),
            [RatelimitHeaders.RESET]: parseInt(headers.get(RatelimitHeaders.RESET) as string)
        };

        if (!res.ok) throw new Error(await res.text() || res.statusText);
        const json = await res.json();
        return json;
    }

    public toQueryString<T = any>(obj: T): string {
        return Object.entries(obj).reduce((p, [k, v]) =>
            p + (p !== '?' ? '&' : '') + (encodeURIComponent(k) + '=' + encodeURIComponent(v)), '?'
        );
    }

    public toEndpointString(endpoint: string, obj: { [key: string]: string }): string {
        return endpoint.replace(/:(\w+)/g, (_, key) => obj[key]);;
    }
}