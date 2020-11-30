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
class ImageScriptEditCommand extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['ist edit'];
        this.label = 'args';
        this.name = 'imagescripttag edit';
        this.metadata = {
            description: 'Edit an existing ImageScript tag',
            examples: ['test' + Math.random().toFixed(3) + ' const image = Image.new(1000, 1000)'],
            usage: '[tag name] [tag content]'
        };
        this.priority = 2;
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            const [tag, content] = this.parseImageScriptArgs(args.args);
            if (!tag) {
                return this.error(context, 'No tag name was specified.');
            }
            const foundTag = yield this.assyst.database.fetchImageScriptTag(tag);
            if (!foundTag || foundTag.owner !== context.userId) {
                return this.error(context, 'This tag either doesn\'t exist or you don\'t own it.');
            }
            const code = yield this.loadCode(context, content);
            if (!code) {
                return this.error(context, 'No tag content was specified.');
            }
            else if (code.length > 10000) {
                return this.error(context, 'ImageScript tags cannot be longer than 10,000 characters.');
            }
            yield this.assyst.database.editImageScriptTag(tag, code, context.userId, foundTag.uses);
            context.editOrReply('Tag edited successfully.');
        });
    }
}
exports.default = ImageScriptEditCommand;
