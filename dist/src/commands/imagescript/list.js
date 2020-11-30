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
class ImageScriptListCommand extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['ist list'];
        this.name = 'imagescripttag list';
        this.metadata = {
            description: 'Fetch a list of all ImageScript tags you own',
            examples: ['me'],
            usage: '[tag name]'
        };
        this.priority = 2;
    }
    run(context) {
        return __awaiter(this, void 0, void 0, function* () {
            const userTags = yield this.assyst.database.fetchUserImageScriptTags(context.userId);
            if (userTags.length === 0) {
                return this.error(context, 'You don\'t own any tags.');
            }
            const lists = utils_1.splitArray(userTags.map(t => [t.name, t.uses]), 10);
            const pages = [];
            for (const page of lists) {
                pages.push(utils_2.Markup.codeblock(utils_1.generateTable({
                    offset: 4,
                    header: ['Name', 'Uses'],
                    rows: page
                }), {
                    language: 'hs'
                }));
            }
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
exports.default = ImageScriptListCommand;
