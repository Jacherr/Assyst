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
class ImageScriptDeleteCommand extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['ist delete'];
        this.label = 'name';
        this.name = 'imagescripttag delete';
        this.metadata = {
            description: 'Delete an ImageScript tag',
            examples: ['test' + Math.random().toFixed(3)],
            usage: '[tag name]'
        };
        this.priority = 2;
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            if (!args.name) {
                return this.error(context, 'No tag name was specified.');
            }
            const tag = yield this.assyst.database.fetchImageScriptTag(args.name);
            if (!tag || tag.owner !== context.userId) {
                return this.error(context, 'This tag either doesn\'t exist or you don\'t own it.');
            }
            this.assyst.database.deleteImageScriptTag(tag.name);
            context.editOrReply('Tag deleted successfully.');
        });
    }
}
exports.default = ImageScriptDeleteCommand;
