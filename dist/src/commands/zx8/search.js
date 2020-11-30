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
class Zx8SearchCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.name = 'zx8 search';
        this.label = 'query';
        this.args = [
            {
                name: 'limit',
                type: Number,
                default: 10
            },
            {
                name: 'ct',
                type: String,
                default: 'all'
            },
            {
                name: 'offset',
                type: Number,
                default: 0
            }
        ];
        this.metadata = {
            description: 'Search the zx8 web scraper',
            examples: ['meme'],
            usage: '[query]'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            const ctAssociations = {
                all: '',
                other: 0,
                image: 1,
                animated: 2,
                gif: 2,
                video: 3,
                html: 4
            };
            if (args.limit > 50) {
                return this.error(context, 'Limit not exceed 50 items.');
            }
            const res = yield this.assyst.zx8.search({
                query: args.query,
                limit: args.limit,
                offset: args.offset,
                ct: ctAssociations[args.ct] || undefined,
                ocr: false
            });
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
exports.default = Zx8SearchCommand;
