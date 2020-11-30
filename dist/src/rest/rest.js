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
const node_fetch_1 = __importDefault(require("node-fetch"));
var Endpoints;
(function (Endpoints) {
    Endpoints["TSU"] = "https://tsu.sh";
    Endpoints["RUST"] = "https://play.rust-lang.org/execute";
})(Endpoints = exports.Endpoints || (exports.Endpoints = {}));
function uploadToTsu(data, contentType) {
    return __awaiter(this, void 0, void 0, function* () {
        return node_fetch_1.default(Endpoints.TSU, {
            headers: {
                'content-type': contentType
            },
            method: 'POST',
            body: data
        }).then(r => r.text());
    });
}
exports.uploadToTsu = uploadToTsu;
function runRustCode(code, options = {}) {
    return __awaiter(this, void 0, void 0, function* () {
        if (!code.includes('fn main('))
            code = `fn main() {\n\t${code}\n}`;
        return node_fetch_1.default(Endpoints.RUST, {
            method: 'POST',
            body: JSON.stringify(Object.assign({ code, channel: 'stable', crateType: 'bin', edition: '2018', mode: 'debug', tests: false }, options))
        })
            .then(x => x.json())
            .then(x => x.error || x.stdout || x.stderr);
    });
}
exports.runRustCode = runRustCode;
