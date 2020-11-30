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
const constants_1 = require("../../constants");
class DuckDuckGoImageCommand extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.label = 'query';
        this.name = 'duckduckgoimage';
        this.aliases = ['ddgi', 'img'];
        this.metadata = {
            description: 'Search Duck Duck Go Images',
            examples: ['cat'],
            usage: '[search query]'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            let results = yield this.fapi.duckDuckGoImages(args.query, {
                safe: yield context.rest.fetchChannel(context.channelId).then(c => c.nsfw)
            });
            if (results.length === 0) {
                return this.error(context, 'No results found');
            }
            const pages = results.map(i => {
                return {
                    embed: {
                        image: {
                            url: i
                        },
                        color: constants_1.EmbedColors.INFO
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
exports.default = DuckDuckGoImageCommand;
