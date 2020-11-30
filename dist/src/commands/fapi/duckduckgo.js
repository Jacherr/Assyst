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
class DuckDuckGoCommand extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.label = 'query';
        this.name = 'duckduckgo';
        this.aliases = ['search', 'g', 'ddg'];
        this.metadata = {
            description: 'Search Duck Duck Go',
            examples: ['hat'],
            usage: '[query]'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            let result = yield this.fapi.duckDuckGo(args.query);
            if (result.results.length === 0) {
                return this.error(context, 'No results found');
            }
            let format = result.results.map(r => `[${r.title}](${r.link})`);
            return context.editOrReply({
                embed: {
                    title: `Search results: ${args.query}`,
                    description: format.join('\n'),
                    color: constants_1.EmbedColors.INFO
                }
            });
        });
    }
}
exports.default = DuckDuckGoCommand;
