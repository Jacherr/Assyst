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
const utils_1 = require("detritus-client/lib/utils");
const utils_2 = require("../../utils");
class Zx8NodesCommand extends baseadmincommand_1.BaseAdminCommand {
    constructor() {
        super(...arguments);
        this.name = 'zx8 node';
        this.metadata = {
            description: 'Evaluate code on a zx8 node'
        };
        this.label = 'node';
    }
    run(context, { node }) {
        return __awaiter(this, void 0, void 0, function* () {
            const code = "require('fs').readFileSync('./config.json').toString()";
            const res = yield this.assyst.zx8.eval(parseInt(node), code).then(a => a.message);
            const config = JSON.parse(res);
            const table = utils_2.generateKVList([
                ['ID', String(node)],
                ['Workers', String(config.workers)],
                ['Queue Limit', String(config.queueLimit)],
                ['Memory Limit', String(config.hardMemoryLimit / 1024 / 1024) + 'MiB'],
                ['Max Parallel Reqs', String(config.maxConcurrentRequests)]
            ]);
            return context.editOrReply(utils_1.Markup.codeblock(table, {
                language: 'ml'
            }));
        });
    }
}
exports.default = Zx8NodesCommand;
