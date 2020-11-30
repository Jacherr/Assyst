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
const constants_1 = require("../../constants");
class Zx8InfoCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.name = 'zx8 info';
        this.metadata = {
            description: 'Get information about the zx8 web scraper'
        };
    }
    run(context) {
        return __awaiter(this, void 0, void 0, function* () {
            const start = Date.now();
            const res = yield this.assyst.zx8.info();
            const time = Date.now() - start;
            const table = utils_1.generateKVList([
                ['URL Queue', res.urlQueue.toLocaleString()],
                ['Total URLs', res.totalURLs.toLocaleString()],
                ['RSS', `${(res.rss / 1024 / 1204).toFixed(2)}MiB`],
                ['Table Size', `${(res.tableSize / 1000).toLocaleString()}GB`],
                ['Indexes Per Second', res.indexesPerSecond.toLocaleString()],
                ['', ''],
                ['Images', res.contentTypes.image.toLocaleString()],
                ['GIFs', res.contentTypes.animated.toLocaleString()],
                ['Videos', res.contentTypes.video.toLocaleString()],
                ['HTML Documents', res.contentTypes.html.toLocaleString()],
                ['Other Documents', res.contentTypes.other.toLocaleString()]
            ]);
            return context.editOrReply({
                embed: {
                    color: constants_1.EmbedColors.ZX8,
                    description: utils_2.Markup.codeblock(table, {
                        language: 'ml'
                    }),
                    footer: {
                        text: `Took ${time}ms`
                    },
                    title: 'zx8 information'
                }
            });
        });
    }
}
exports.default = Zx8InfoCommand;
