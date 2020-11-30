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
const basecommand_1 = require("../basecommand");
const utils_1 = require("../../utils");
const rest_1 = require("../../rest/rest");
const utils_2 = require("detritus-client/lib/utils");
class RustCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['rs'];
        this.args = [
            {
                name: 'channel',
                type: String,
                default: 'stable'
            },
            {
                name: 'backtrace',
                type: Boolean
            },
            {
                name: 'cratetype',
                type: String,
                default: 'bin'
            },
            {
                name: 'edition',
                type: String,
                default: '2018'
            },
            {
                name: 'mode',
                type: String,
                default: 'debug'
            },
            {
                name: 'tests',
                type: Boolean,
                default: false
            }
        ];
        this.label = 'code';
        this.name = 'rust';
        this.metadata = {
            description: 'Execute rust code',
            examples: ['println!("hello")'],
            usage: '[code]'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            if (!args.code) {
                return this.error(context, 'Provide some code to execute.');
            }
            const codeToExecute = utils_1.parseCodeblocks(args.code);
            const result = yield rest_1.runRustCode(codeToExecute, {
                channel: args.channel,
                backtrace: args.backtrace,
                crateType: args.cratetype,
                edition: args.edition,
                mode: args.mode,
                tests: args.tests
            });
            return context.editOrReply(utils_2.Markup.codeblock(result, {
                language: 'rs'
            }));
        });
    }
}
exports.default = RustCommand;
