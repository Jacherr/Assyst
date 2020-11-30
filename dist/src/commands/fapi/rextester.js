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
const utils_2 = require("detritus-client/lib/utils");
class RextesterCommand extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.label = 'code';
        this.name = 'rextester';
        this.aliases = ['rex'];
        this.args = [
            {
                name: 'lang',
                type: String,
                default: 'node'
            }
        ];
        this.metadata = {
            description: 'Run code on rextester',
            examples: ['console.log(1)', 'print(1) -lang py'],
            usage: '[code] <-lang language>'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            const code = utils_1.parseCodeblocks(args.code);
            let result = yield this.fapi.rexTester(args.lang, code).then((a) => a === null || a === void 0 ? void 0 : a.toString());
            return context.reply(utils_2.Markup.codeblock(result !== null && result !== void 0 ? result : 'Empty response', {
                language: args.lang
            }));
        });
    }
}
exports.default = RextesterCommand;
