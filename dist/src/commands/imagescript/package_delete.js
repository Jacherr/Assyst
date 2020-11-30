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
class ImageScriptPackageDeleteCommand extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['ist package delete'];
        this.label = 'name';
        this.name = 'imagescripttag package delete';
        this.metadata = {
            description: 'Delete an ImageScript package',
            examples: ['test' + Math.random().toFixed(3)],
            usage: '[package name]'
        };
        this.priority = 2;
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            if (!args.name) {
                return this.error(context, 'No package name was specified.');
            }
            const tag = yield this.assyst.database.fetchImageScriptPackage(args.name);
            if (!tag || tag.owner !== context.userId) {
                return this.error(context, 'This package either doesn\'t exist or you don\'t own it.');
            }
            this.assyst.database.deleteImageScriptPackage(tag.name);
            context.editOrReply('Package deleted successfully.');
        });
    }
}
exports.default = ImageScriptPackageDeleteCommand;
