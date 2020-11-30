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
const utils_1 = require("detritus-client/lib/utils");
const util_1 = require("util");
const baseadmincommand_1 = require("../baseadmincommand");
const utils_2 = require("../../utils");
const config_json_1 = require("../../../config.json");
const tokensList = Object.values(config_json_1.tokens).join('|');
// eslint-disable-next-line no-useless-escape
const TOKEN_REGEX = new RegExp(tokensList.replace(/([\.\{\}\(\)\*\+\-\=\!\?\^\$])/g, '\\$1'), 'g');
const TIMEOUT = 10000;
const fns = [];
const suite = {
    add(data) {
        fns.push(data);
        return this;
    },
    run(data) {
        const results = [];
        for (const test of fns) {
            const before = Date.now();
            for (let i = 0; i < data.iterations; ++i)
                test.fn(i);
            results.push({ name: test.name, time: Date.now() - before });
        }
        return results;
    }
};
function bm(...funcs) {
    let i = 0;
    let out;
    for (const arg of funcs) {
        if (typeof arg === 'function') {
            suite.add({ name: i, fn: arg });
        }
        else {
            out = suite.run({ iterations: parseInt(arg, 10) });
            break;
        }
        i++;
    }
    fns.splice(0, fns.length);
    return out === null || out === void 0 ? void 0 : out.map(i => i.time + 'ms');
}
class EvalCommand extends baseadmincommand_1.BaseAdminCommand {
    constructor() {
        super(...arguments);
        //@ts-ignore
        this.args = [
            {
                name: 'async',
                type: Boolean,
                default: false
            },
            {
                name: 'attach',
                type: Boolean,
                default: false
            },
            {
                name: 'depth',
                default: '0',
                type: Number
            },
            {
                name: 'noreply',
                type: Boolean,
                default: false
            }
        ];
        this.label = 'code';
        this.name = 'e';
        this.metadata = {
            description: 'Evaluate JavaScript',
            examples: ['1+1'],
            usage: '[code]'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            let evaled;
            const code = utils_2.parseCodeblocks(args.code);
            try {
                if (!args.async) {
                    evaled = yield Promise.resolve(eval(code));
                }
                else {
                    evaled = yield Promise.resolve(eval(`(async () => {\n${code}\n})()`));
                }
            }
            catch (e) {
                return context.editOrReply(utils_1.Markup.codeblock(e.message || e.stack || e.toString(), { limit: 1990, language: 'js' }));
            }
            if (args.attach && !args.noreply) {
                let extension = 'txt';
                if (Buffer.isBuffer(evaled))
                    extension = 'png';
                else if (typeof evaled === 'object') {
                    evaled = util_1.inspect(evaled, { depth: args.depth, showHidden: true });
                }
                else {
                    evaled = String(evaled);
                }
                if (typeof evaled === 'string')
                    evaled = evaled.replace(TOKEN_REGEX, '');
                return context.editOrReply({ file: { value: evaled, filename: `eval.${extension}` } });
            }
            else if (!args.noreply) {
                if (typeof evaled === 'object') {
                    evaled = util_1.inspect(evaled, { depth: args.depth, showHidden: true });
                }
                else {
                    evaled = String(evaled);
                }
                evaled = evaled.replace(TOKEN_REGEX, '');
                return context.editOrReply(utils_1.Markup.codeblock(evaled, {
                    language: 'js',
                    limit: 1990
                }));
            }
        });
    }
}
exports.default = EvalCommand;
