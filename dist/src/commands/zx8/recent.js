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
class Zx8RecentCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.name = 'zx8 recent';
        this.metadata = {
            description: 'Get recently indexed urls from zx8'
        };
    }
    run(context) {
        return __awaiter(this, void 0, void 0, function* () {
            const res = yield this.assyst.zx8.recentIndexes();
            if (res.length === 0) {
                return this.error(context, 'No results found');
            }
            const rows = res.map((r, i) => {
                return [i, r.url.trim().slice(0, 125), r.lastStatus];
            });
            const rawPages = utils_1.flat(rows, 5);
            const formattedPages = rawPages.map(p => utils_1.generateTable({
                offset: 4,
                header: ['#', 'URL', 'Status'],
                rows: p
            }));
            const codeblockedPages = formattedPages.map(p => {
                return utils_2.Markup.codeblock(p, {
                    language: 'md'
                });
            });
            return yield this.assyst.paginator.createReactionPaginator({
                pages: codeblockedPages,
                message: context
            });
        });
    }
}
exports.default = Zx8RecentCommand;
