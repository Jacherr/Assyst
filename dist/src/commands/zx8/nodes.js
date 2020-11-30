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
const utils_2 = require("detritus-client/lib/utils");
class Zx8NodesCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.name = 'zx8 nodes';
        this.metadata = {
            description: 'Get information about the zx8 web scraper\'s nodes'
        };
    }
    run(context) {
        return __awaiter(this, void 0, void 0, function* () {
            const res = yield this.assyst.zx8.nodes();
            const rows = res.map((r, index) => {
                return [index, `${r.ping}ms`, `${(r.memory / 1024 / 1024).toFixed(2)}MiB`, r.available ? 'Yes' : 'No', r.queue];
            });
            const table = utils_1.generateTable({
                offset: 4,
                header: ['#', 'Ping', 'RSS', 'Available', 'URL Queue'],
                rows
            });
            return context.editOrReply(utils_2.Markup.codeblock(table, {
                language: 'md'
            }));
        });
    }
}
exports.default = Zx8NodesCommand;
