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
const utils_1 = require("detritus-client/lib/utils");
class ImageScriptPackageRawCommand extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['ist package raw'];
        this.args = [
            {
                name: 'upload',
                default: false,
                type: Boolean
            }
        ];
        this.label = 'name';
        this.name = 'imagescripttag package raw';
        this.metadata = {
            description: 'Fetch the raw content of an ImageScript package',
            examples: ['random'],
            usage: '[tag name]'
        };
        this.priority = 2;
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            if (!args.name) {
                return this.error(context, 'No package name was specified.');
            }
            const code = yield this.assyst.database.fetchImageScriptPackage(args.name);
            if (!code) {
                return this.error(context, 'No package with this name was found.');
            }
            let output;
            if (code.content.length > 1995 || args.attach) {
                output = {
                    content: yield this.uploadFile(code.content, 'application/javascript')
                };
            }
            else {
                output = {
                    content: utils_1.Markup.codeblock(code.content, {
                        language: 'js'
                    })
                };
            }
            return context.editOrReply(output);
        });
    }
}
exports.default = ImageScriptPackageRawCommand;
