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
Object.defineProperty(exports, "__esModule", { value: true });
const basefapicommand_1 = require("../basefapicommand");
const utils_1 = require("../../utils");
const types_1 = require("fapi-client/JS/src/types");
const utils_2 = require("detritus-client/lib/utils");
class FapiCommand extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.name = 'fapi';
        this.metadata = {
            description: 'Get fAPI info for Assyst'
        };
    }
    run(context) {
        return __awaiter(this, void 0, void 0, function* () {
            const limits = this.fapi.ratelimits;
            const table = utils_1.generateKVList([
                [
                    types_1.ReturnHeaders.RATELIMIT_LIMIT,
                    (() => {
                        var _a;
                        const result = (_a = limits[types_1.ReturnHeaders.RATELIMIT_LIMIT]) === null || _a === void 0 ? void 0 : _a.toString();
                        if (result)
                            return parseInt(result).toLocaleString() + 'ms';
                        return 'none';
                    })()
                ],
                [
                    types_1.ReturnHeaders.RATELIMIT_REMAINING,
                    (() => {
                        const result = limits[types_1.ReturnHeaders.RATELIMIT_REMAINING];
                        if (result)
                            return Math.floor(result).toLocaleString() + 'ms';
                        return 'none';
                    })()
                ],
                [
                    types_1.ReturnHeaders.RATELIMIT_RESET,
                    (() => {
                        var _a;
                        const result = (_a = limits[types_1.ReturnHeaders.RATELIMIT_RESET]) === null || _a === void 0 ? void 0 : _a.toString();
                        if (result)
                            return parseInt(result).toLocaleString() + 's';
                        return 'none';
                    })()
                ],
                [
                    'timeout',
                    this.fapi.timeout.toString() + 'ms'
                ],
                [
                    'ping',
                    (yield this.fapi.ping()) + 'ms'
                ]
            ]);
            return context.editOrReply(utils_2.Markup.codeblock(table, {
                language: 'ml'
            }));
        });
    }
}
exports.default = FapiCommand;
