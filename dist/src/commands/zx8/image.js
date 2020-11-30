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
const types_1 = require("../../rest/zx8/types");
const constants_1 = require("../../constants");
class Zx8ImageCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['zx8 img', 'zx8 i'];
        this.name = 'zx8 image';
        this.label = 'query';
        this.args = [
            {
                name: 'limit',
                type: Number,
                default: 10
            },
            {
                name: 'ocr',
                type: Boolean,
                default: false
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
            if (args.limit > 100) {
                return this.error(context, 'Limit must 100 or less');
            }
            const res = yield this.assyst.zx8.search({
                query: args.query,
                limit: args.limit,
                offset: args.offset,
                ct: types_1.ContentTypes.IMAGE,
                ocr: args.ocr
            });
            if (res.length === 0) {
                return this.error(context, 'No results found');
            }
            const items = res.map(r => r.url);
            const pages = items.map(i => {
                return {
                    embed: {
                        image: {
                            url: i
                        },
                        color: constants_1.EmbedColors.ZX8
                    }
                };
            });
            const paginator = yield this.assyst.paginator.createReactionPaginator({
                pages,
                message: context
            });
            this.assyst.replies.set(context.messageId, {
                command: this,
                context,
                reply: paginator.commandMessage
            });
        });
    }
}
exports.default = Zx8ImageCommand;
