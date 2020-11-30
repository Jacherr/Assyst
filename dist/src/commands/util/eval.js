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
const basecommand_1 = require("../basecommand");
const util_1 = require("util");
const node_fetch_1 = __importDefault(require("node-fetch"));
const utils_1 = require("detritus-client/lib/utils");
const types_1 = require("fapi-client/JS/src/types");
const config_json_1 = require("../../../config.json");
class EvalCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.label = 'code';
        this.name = 'eval';
        this.metadata = {
            description: 'Evaluate code',
            examples: ['1+1', 'process.versions'],
            usage: '[code]'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            let result = yield this.runCode(args.code || 'undefined');
            try {
                result = JSON.parse(result);
                result = util_1.inspect(result, {
                    showHidden: true,
                    depth: 1
                });
            }
            catch (_a) { }
            context.editOrReply(utils_1.Markup.codeblock(result || 'undefined', {
                language: 'js',
                limit: 1990
            }));
        });
    }
    runCode(code) {
        return __awaiter(this, void 0, void 0, function* () {
            const res = yield node_fetch_1.default(config_json_1.evalUrl, {
                method: types_1.HttpMethods.POST,
                body: JSON.stringify({
                    code
                }),
                headers: {
                    'content-type': 'application/json'
                }
            });
            if (!res.ok)
                throw new Error(`${res.status}: ${res.statusText}`);
            return res.json().then(json => json.message);
        });
    }
}
exports.default = EvalCommand;
