"use strict";
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (Object.hasOwnProperty.call(mod, k)) result[k] = mod[k];
    result["default"] = mod;
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
const Types = __importStar(require("./types"));
const baserestclient_1 = require("../baserestclient");
const config_json_1 = require("../../../config.json");
exports.BASE_URL = 'https://zx8.jacher.io';
exports.VERSION = 'v1';
exports.API_URL = exports.BASE_URL + '/api/' + exports.VERSION;
var Endpoints;
(function (Endpoints) {
    Endpoints["EVAL"] = "/rpc/:node";
    Endpoints["INFO"] = "/info";
    Endpoints["NODES"] = "/nodes";
    Endpoints["RECENT"] = "/recent";
    Endpoints["SEARCH"] = "/search";
})(Endpoints = exports.Endpoints || (exports.Endpoints = {}));
class Zx8 extends baserestclient_1.BaseRestClient {
    constructor() {
        super(exports.API_URL);
    }
    eval(nodeId, code) {
        return this.post(this.toEndpointString(Endpoints.EVAL, {
            node: String(nodeId)
        }), {
            authorization: config_json_1.zx8Token
        }, {
            code
        });
    }
    info() {
        return this.get(Endpoints.INFO);
    }
    nodes() {
        return this.get(Endpoints.NODES);
    }
    randomAudio() {
        const offset = Math.floor((Math.random() * 2000) || 0);
        return this.search({
            ct: Types.ContentTypes.OTHER,
            offset,
            query: '.mp3',
            limit: 5
        });
    }
    randomImage() {
        const offset = Math.floor((Math.random() * 1000000) || 0);
        return this.search({
            ct: Types.ContentTypes.IMAGE,
            offset,
            query: '.',
            limit: 5
        });
    }
    randomHtml() {
        const offset = Math.floor((Math.random() * 15000000) || 0);
        return this.search({
            ct: Types.ContentTypes.HTML,
            offset,
            query: '.',
            limit: 5
        });
    }
    randomVideo() {
        const offset = Math.floor((Math.random() * 3000) || 0);
        return this.search({
            ct: Types.ContentTypes.VIDEO,
            offset,
            query: '.',
            limit: 5
        });
    }
    recentIndexes() {
        return this.get(Endpoints.RECENT);
    }
    search(data) {
        const fd = typeof data === 'string' ? {
            query: data
        } : data;
        return this.get(Endpoints.SEARCH + this.toQueryString(fd));
    }
}
exports.Zx8 = Zx8;
