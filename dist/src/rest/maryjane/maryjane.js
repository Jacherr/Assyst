"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const config_json_1 = require("../../../config.json");
const baserestclient_1 = require("../baserestclient");
exports.BASE_URL = config_json_1.maryjaneUrl;
var Endpoints;
(function (Endpoints) {
    Endpoints["GUILD"] = "/guild/:id";
    Endpoints["USER"] = "/user/:id";
})(Endpoints = exports.Endpoints || (exports.Endpoints = {}));
class Maryjane extends baserestclient_1.BaseRestClient {
    constructor() {
        super(exports.BASE_URL);
    }
    guild(id) {
        return this.get(this.toEndpointString(Endpoints.GUILD, { id }));
    }
    user(id) {
        return this.get(this.toEndpointString(Endpoints.USER, { id }));
    }
}
exports.Maryjane = Maryjane;
