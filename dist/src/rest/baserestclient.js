"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const types_1 = require("./types");
const node_fetch_1 = __importDefault(require("node-fetch"));
const types_2 = require("fapi-client/JS/src/types");
class BaseRestClient {
    constructor(baseUrl) {
        this.timeout = 20000;
        this.baseUrl = baseUrl;
    }
    get(endpoint) {
        return __awaiter(this, void 0, void 0, function* () {
            const res = yield Promise.race([
                node_fetch_1.default(this.baseUrl + endpoint),
                new Promise((resolve, reject) => {
                    setTimeout(() => reject(new Error(`Timeout after ${this.timeout}ms`)), this.timeout);
                })
            ]);
            const headers = res.headers;
            this.ratelimits = {
                [types_1.RatelimitHeaders.LIMIT]: parseInt(headers.get(types_1.RatelimitHeaders.LIMIT)),
                [types_1.RatelimitHeaders.REMAINING]: parseInt(headers.get(types_1.RatelimitHeaders.REMAINING)),
                [types_1.RatelimitHeaders.RESET]: parseInt(headers.get(types_1.RatelimitHeaders.RESET))
            };
            if (!res.ok)
                throw new Error((yield res.text()) || res.statusText);
            const json = yield res.json();
            return json;
        });
    }
    post(endpoint, requestHeaders, body) {
        return __awaiter(this, void 0, void 0, function* () {
            const res = yield Promise.race([
                node_fetch_1.default(this.baseUrl + endpoint, {
                    method: types_2.HttpMethods.POST,
                    body: JSON.stringify(body),
                    headers: Object.assign({ 'content-type': 'application/json' }, requestHeaders)
                }),
                new Promise((resolve, reject) => {
                    setTimeout(() => reject(new Error(`Timeout after ${this.timeout}ms`)), this.timeout);
                })
            ]);
            const headers = res.headers;
            this.ratelimits = {
                [types_1.RatelimitHeaders.LIMIT]: parseInt(headers.get(types_1.RatelimitHeaders.LIMIT)),
                [types_1.RatelimitHeaders.REMAINING]: parseInt(headers.get(types_1.RatelimitHeaders.REMAINING)),
                [types_1.RatelimitHeaders.RESET]: parseInt(headers.get(types_1.RatelimitHeaders.RESET))
            };
            if (!res.ok)
                throw new Error((yield res.text()) || res.statusText);
            const json = yield res.json();
            return json;
        });
    }
    toQueryString(obj) {
        return Object.entries(obj).reduce((p, [k, v]) => p + (p !== '?' ? '&' : '') + (encodeURIComponent(k) + '=' + encodeURIComponent(v)), '?');
    }
    toEndpointString(endpoint, obj) {
        return endpoint.replace(/:(\w+)/g, (_, key) => obj[key]);
        ;
    }
}
exports.BaseRestClient = BaseRestClient;
