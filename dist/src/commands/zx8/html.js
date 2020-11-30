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
class Zx8InfoCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['zx8 page'];
        this.name = 'zx8 html';
        this.label = 'query';
        this.metadata = {
            description: 'Search for and screenshot a single html web page',
            examples: ['reddit'],
            usage: '[query]'
        };
    }
    run(context, args) {
        var _a, _b;
        return __awaiter(this, void 0, void 0, function* () {
            yield context.triggerTyping();
            const { query } = args;
            const result = yield this.assyst.zx8.search({
                ct: types_1.ContentTypes.HTML,
                query
            });
            if (result.length === 0) {
                return this.error(context, 'No results found');
            }
            let screenshot;
            try {
                screenshot = yield this.assyst.screenshot(result[0].url, (_b = (_a = context.channel) === null || _a === void 0 ? void 0 : _a.nsfw) !== null && _b !== void 0 ? _b : false, 0);
            }
            catch (e) {
                return this.error(context, `${result[0].url}\n\n${e.message}`);
            }
            return context.editOrReply({
                content: `<${result[0].url}>`,
                file: {
                    filename: 'screenshot.png',
                    value: screenshot
                }
            });
        });
    }
}
exports.default = Zx8InfoCommand;
