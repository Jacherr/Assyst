"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Client = void 0;
const url_1 = require("url");
const node_fetch_1 = require("node-fetch");
const constants_1 = require("./constants");
const request_1 = require("./request");
const defaultClientOptions = Object.freeze({
    headers: {
        [constants_1.HTTPHeaders.USER_AGENT]: `detritus-rest (${constants_1.Package.URL}, ${constants_1.Package.VERSION})`,
    },
});
class Client {
    constructor(options) {
        options = Object.assign({}, defaultClientOptions, options);
        this.agent = options.agent;
        this.baseUrl = '';
        if (options.baseUrl) {
            if (options.baseUrl instanceof url_1.URL) {
                this.baseUrl = options.baseUrl;
            }
            else {
                this.baseUrl = new url_1.URL(options.baseUrl);
            }
        }
        this.headers = request_1.createHeaders(options.headers);
        for (let key in defaultClientOptions.headers) {
            if (!this.headers.has(key)) {
                const value = defaultClientOptions.headers[key];
                this.headers.set(key, value);
            }
        }
    }
    createRequest(info, init) {
        // inject base options from the client here
        init = Object.assign({
            agent: this.agent,
        }, init);
        let url;
        if (typeof (info) === 'string' || info instanceof url_1.URL) {
            url = info;
        }
        else {
            init = Object.assign({}, info, init);
            if (init.url || (this.baseUrl && (init.path || (init.route && init.route.path)))) {
                url = init.url || this.baseUrl;
            }
            else {
                if (this.baseUrl) {
                    throw new Error('A Path is required if using the base URL from the client');
                }
                else {
                    throw new Error('A URL is required if there is no base URL');
                }
            }
        }
        if (init.headers) {
            init.headers = request_1.createHeaders(init.headers);
            for (let [key, value] of this.headers) {
                if (!init.headers.has(key)) {
                    init.headers.set(key, value);
                }
            }
        }
        else {
            init.headers = new node_fetch_1.Headers(this.headers);
        }
        return new request_1.Request(url, init);
    }
    async request(info, init) {
        const request = this.createRequest(info, init);
        return request.send();
    }
    async delete(info, init) {
        init = Object.assign({}, init, { method: constants_1.HTTPMethods.DELETE });
        return this.request(info, init);
    }
    async get(info, init) {
        init = Object.assign({}, init, { method: constants_1.HTTPMethods.GET });
        return this.request(info, init);
    }
    async head(info, init) {
        init = Object.assign({}, init, { method: constants_1.HTTPMethods.HEAD });
        return this.request(info, init);
    }
    async options(info, init) {
        init = Object.assign({}, init, { method: constants_1.HTTPMethods.OPTIONS });
        return this.request(info, init);
    }
    async patch(info, init) {
        init = Object.assign({}, init, { method: constants_1.HTTPMethods.PATCH });
        return this.request(info, init);
    }
    async post(info, init) {
        init = Object.assign({}, init, { method: constants_1.HTTPMethods.POST });
        return this.request(info, init);
    }
    async put(info, init) {
        init = Object.assign({}, init, { method: constants_1.HTTPMethods.PUT });
        return this.request(info, init);
    }
}
exports.Client = Client;
