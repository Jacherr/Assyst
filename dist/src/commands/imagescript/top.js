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
const utils_1 = require("../../utils");
const utils_2 = require("detritus-client/lib/utils");
class ImageScriptTopCommand extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['ist top'];
        this.name = 'imagescripttag top';
        this.metadata = {
            description: 'Fetch the info of an ImageScript tag',
            examples: ['me'],
            usage: '[tag name]'
        };
        this.priority = 2;
    }
    run(context) {
        return __awaiter(this, void 0, void 0, function* () {
            const topTags = yield this.assyst.database.fetchTopImageScriptTags();
            const rows = topTags.map(t => [t.name, t.uses, t.owner]);
            const table = utils_1.generateTable({
                offset: 4,
                header: ['Name', 'Uses', 'Owner'],
                rows
            });
            return context.editOrReply(utils_2.Markup.codeblock(table, {
                language: 'hs'
            }));
        });
    }
}
exports.default = ImageScriptTopCommand;
