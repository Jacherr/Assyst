"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Response = void 0;
const node_fetch_1 = require("node-fetch");
const constants_1 = require("./constants");
class Response {
    constructor(request, response, took = 0) {
        this._body = null;
        this.fetchResponse = response;
        this.request = request;
        this.took = took;
        Object.defineProperties(this, {
            _body: { enumerable: false },
        });
    }
    get body() {
        return this.fetchResponse.body;
    }
    get bodyUsed() {
        return this.fetchResponse.bodyUsed;
    }
    get headers() {
        return this.fetchResponse.headers;
    }
    get ok() {
        return this.fetchResponse.ok;
    }
    get redirected() {
        return this.fetchResponse.redirected;
    }
    get size() {
        return this.fetchResponse.size;
    }
    get status() {
        return this.fetchResponse.status;
    }
    get statusCode() {
        return this.status;
    }
    get statusText() {
        return this.fetchResponse.statusText;
    }
    get url() {
        return this.fetchResponse.url;
    }
    async arrayBuffer() {
        const { buffer, byteOffset, byteLength } = await this.buffer();
        return buffer.slice(byteOffset, byteOffset + byteLength);
    }
    async blob() {
        const contentType = this.headers.get(constants_1.HTTPHeaders.CONTENT_TYPE) || (this.body && this.body.type) || '';
        const buffer = await this.buffer();
        return new node_fetch_1.Blob([buffer], { type: contentType });
    }
    async buffer() {
        if (this._body) {
            return this._body;
        }
        this._body = this.fetchResponse.buffer();
        return this._body = await this._body;
    }
    async json() {
        const text = await this.text();
        if (text) {
            return JSON.parse(text);
        }
        return null;
    }
    async text() {
        return (await this.buffer()).toString();
    }
    clone() {
        return new Response(this.request, this.fetchResponse);
    }
    toString() {
        return this.request.toString();
    }
}
exports.Response = Response;
Object.defineProperties(Response.prototype, {
    arrayBuffer: { enumerable: true },
    blob: { enumerable: true },
    body: { enumerable: true },
    bodyUsed: { enumerable: true },
    clone: { enumerable: true },
    headers: { enumerable: true },
    json: { enumerable: true },
    ok: { enumerable: true },
    redirected: { enumerable: true },
    status: { enumerable: true },
    statusText: { enumerable: true },
    text: { enumerable: true },
    url: { enumerable: true },
});
