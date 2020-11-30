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
const baseadmincommand_1 = require("../baseadmincommand");
const utils_1 = require("../../utils");
const utils_2 = require("detritus-client/lib/utils");
class Zx8NodesCommand extends baseadmincommand_1.BaseAdminCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['zx8 rpc'];
        this.name = 'zx8 eval';
        this.metadata = {
            description: 'Evaluate code on a zx8 node'
        };
        this.label = 'args';
    }
    run(context, { args }) {
        return __awaiter(this, void 0, void 0, function* () {
            const splitArgs = args.split(/ +/g);
            const node = splitArgs[0];
            const code = utils_1.parseCodeblocks(splitArgs.slice(1).join(' ')).trim();
            const res = yield this.assyst.zx8.eval(parseInt(node), code).then(a => a.message);
            return context.editOrReply(utils_2.Markup.codeblock(res, {
                language: 'js'
            }));
        });
    }
}
exports.default = Zx8NodesCommand;
