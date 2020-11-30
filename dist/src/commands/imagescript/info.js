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
class ImageScriptInfoCommand extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['ist info'];
        this.label = 'name';
        this.name = 'imagescripttag info';
        this.metadata = {
            description: 'Fetch the info of an ImageScript tag',
            examples: ['me'],
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
            if (!tag) {
                return this.error(context, 'No tag with this name was found.');
            }
            const output = utils_1.generateKVList([
                ['Name', tag.name],
                ['Owner', yield context.rest.fetchUser(tag.owner).then(u => `${u.username}#${u.discriminator}`)],
                ['Uses', tag.uses.toString()]
            ]);
            return context.editOrReply(utils_2.Markup.codeblock(output, {
                language: 'ml'
            }));
        });
    }
}
exports.default = ImageScriptInfoCommand;
