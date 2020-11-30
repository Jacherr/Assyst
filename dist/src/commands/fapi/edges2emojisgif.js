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
class Edges2EmojisGifCommand extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.label = 'url';
        this.name = 'edges2emojisgif';
        this.metadata = {
            description: 'Edges 2 Emojis Gif',
            examples: ['https://link.to.my/image.png'],
            usage: '[url|attachment]'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            const url = yield this.getUrlFromChannel(context, args.url);
            if (!url) {
                return this.error(context, 'No valid URL was found... Please use an attachment or valid image URL');
            }
            const res = yield this.fapi.edges2EmojisGif(url);
            return context.editOrReply({
                file: {
                    filename: 'edges2emojisgif.gif',
                    value: res
                }
            });
        });
    }
}
exports.default = Edges2EmojisGifCommand;
