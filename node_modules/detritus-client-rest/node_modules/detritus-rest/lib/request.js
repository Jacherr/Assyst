"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.createHeaders = exports.appendQuery = exports.Request = void 0;
const url_1 = require("url");
const FormData = require("form-data");
const node_fetch_1 = require("node-fetch");
const constants_1 = require("./constants");
const is_1 = require("./is");
const response_1 = require("./response");
const route_1 = require("./route");
class Request extends node_fetch_1.Request {
    constructor(info, init) {
        init = Object.assign({ jsonify: true }, init);
        let url;
        if (typeof (info) === 'string' || info instanceof url_1.URL) {
            url = new url_1.URL('', info);
        }
        else {
            init = Object.assign({}, info, init);
            if (init.url) {
                url = new url_1.URL('', init.url);
            }
            else {
                throw new Error('A URL is required.');
            }
        }
        init.method = init.method || constants_1.HTTPMethods.GET;
        let route = null;
        if (init.route || init.path) {
            if (init.route instanceof route_1.Route) {
                route = init.route;
            }
            else {
                if (init.route) {
                    route = new route_1.Route(init.route.method || init.method, init.route.path || init.path, init.route.params);
                }
                else {
                    route = new route_1.Route(init.method, init.path);
                }
            }
            init.method = route.method;
        }
        init.method = init.method.toUpperCase();
        if (route) {
            if (url.pathname.endsWith('/') && route.urlPath.startsWith('/')) {
                url.pathname += route.urlPath.slice(1);
            }
            else {
                url.pathname += route.urlPath;
            }
        }
        if (init.query) {
            for (let key in init.query) {
                appendQuery(url, key, init.query[key]);
            }
        }
        init.headers = createHeaders(init.headers);
        let body;
        if (is_1.isFormData(init.body)) {
            body = init.body;
        }
        if (((init.body !== undefined && init.body !== null) && init.multipart) || (init.files && init.files.length)) {
            // convert the body to form-data if `init.body` is non-null and multipart is true OR if theres any files passed in
            if (!body) {
                body = new FormData();
            }
            if (init.files && init.files.length) {
                for (let key in init.files) {
                    const file = init.files[key];
                    body.append(file.key || `file${key}`, file.value, file);
                }
            }
            if (init.body !== undefined && init.body !== null && init.body !== body) {
                if (is_1.isURLSearchParameters(init.body)) {
                    // go through the keys and add it to the form-data
                    for (let [key, value] of init.body) {
                        body.append(key, value);
                    }
                }
                else if (is_1.isBlob(init.body)) {
                    // add it as a file?
                }
                else {
                    if ((init.multipart || !init.jsonify) && typeof (init.body) === 'object') {
                        for (let key in init.body) {
                            body.append(key, init.body[key]);
                        }
                    }
                    else {
                        // If an object is passed in as the body with files, but multipart isn't true or jsonify is false, encode it to json
                        const key = 'payload_json';
                        body.append(key, JSON.stringify(init.body), { contentType: constants_1.ContentTypes.APPLICATION_JSON });
                    }
                }
            }
        }
        else if (init.body !== undefined) {
            if (init.jsonify) {
                init.headers.set(constants_1.HTTPHeaders.CONTENT_TYPE, constants_1.ContentTypes.APPLICATION_JSON);
                body = JSON.stringify(init.body);
            }
            else {
                body = init.body;
            }
        }
        init.body = body;
        super(url, init);
        this.route = route;
    }
    get parsedUrl() {
        const url = this.url;
        if (url instanceof url_1.URL) {
            return url;
        }
        return new url_1.URL(url);
    }
    clone() {
        return new Request(this);
    }
    async send() {
        const now = Date.now();
        const response = await node_fetch_1.default(this.url, this);
        return new response_1.Response(this, response, Date.now() - now);
    }
    toString() {
        return `${this.method}-${url_1.format(this.url)}`;
    }
}
exports.Request = Request;
function appendQuery(url, key, value) {
    if (value === undefined) {
        return;
    }
    if (Array.isArray(value)) {
        for (let v of value) {
            appendQuery(url, key, v);
        }
    }
    else {
        if (typeof (value) !== 'string') {
            value = String(value);
        }
        url.searchParams.append(key, value);
    }
}
exports.appendQuery = appendQuery;
function createHeaders(old) {
    if (old instanceof node_fetch_1.Headers || (old && typeof (old[Symbol.iterator]) === 'function')) {
        return new node_fetch_1.Headers(old);
    }
    else if (old) {
        // go through and pick out the undefined
        const headers = new node_fetch_1.Headers();
        for (let key in old) {
            const value = old[key];
            if (value !== undefined) {
                headers.append(key, value);
            }
        }
        return headers;
    }
    return new node_fetch_1.Headers();
}
exports.createHeaders = createHeaders;
