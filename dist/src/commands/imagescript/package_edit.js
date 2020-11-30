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
const config_json_1 = require("../../../config.json");
class ImageScriptPackageEditCommand extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['ist package edit'];
        this.label = 'args';
        this.name = 'imagescripttag package edit';
        this.metadata = {
            description: 'Edit an existing ImageScript package',
            examples: ['test' + Math.random().toFixed(3) + ' globalThis.a = 1'],
            usage: '[package name] [package content]'
        };
        this.priority = 3;
    }
    onBefore(context) {
        const _super = Object.create(null, {
            onBefore: { get: () => super.onBefore }
        });
        return __awaiter(this, void 0, void 0, function* () {
            if (!config_json_1.packageWhitelist.includes(context.userId) && !context.user.isClientOwner)
                return false;
            return _super.onBefore.call(this, context);
        });
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            const [packageName, content] = this.parseImageScriptArgs(args.args);
            if (!packageName) {
                return this.error(context, 'No package name was specified.');
            }
            const foundTag = yield this.assyst.database.fetchImageScriptPackage(packageName);
            if (!foundTag || foundTag.owner !== context.userId) {
                return this.error(context, 'This package either doesn\'t exist or you don\'t own it.');
            }
            const code = yield this.loadCode(context, content);
            if (!code) {
                return this.error(context, 'No package content was specified.');
            }
            yield this.assyst.database.editImageScriptPackage(packageName, code.trim(), context.userId);
            context.editOrReply('Package edited successfully.');
        });
    }
}
exports.default = ImageScriptPackageEditCommand;
