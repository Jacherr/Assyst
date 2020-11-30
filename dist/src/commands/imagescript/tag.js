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
class ImageScriptTagCommand extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['ist'];
        this.args = [
            {
                name: 'm',
                type: Boolean,
                default: false
            },
            {
                name: 'upload',
                type: Boolean,
                default: false
            }
        ];
        this.label = 'args';
        this.name = 'imagescripttag';
        this.metadata = {
            description: 'Run an ImageScript tag',
            examples: ['me'],
            usage: '[tag name] <tag args>'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            const [tag, tagArgs] = this.parseImageScriptArgs(args.args);
            if (!tag) {
                return this.error(context, 'No tag name was specified.');
            }
            const code = yield this.assyst.database.fetchImageScriptTag(tag);
            if (!code) {
                return this.error(context, 'No tag with this name was found.');
            }
            code.content = yield this.injectImageScriptPackages(code.content);
            let response;
            try {
                response = yield this.fapi.imageScript(code.content, {
                    avatar: context.user.avatarUrl + '?size=1024',
                    args: tagArgs
                });
            }
            catch (e) {
                return context.editOrReply(e.message);
            }
            const guildAttachmentLimitBytes = yield context.rest.fetchGuild(context.guildId).then(g => g.maxAttachmentSize);
            let output = {};
            if (args.m) {
                output.content = [
                    `**CPU Time**: \`${response.cpuTime.toFixed(1)}ms\``,
                    `**Wall Time**: \`${response.wallTime.toFixed(1)}ms\``,
                    `**Memory Usage**: \`${response.memoryUsage.toFixed(1)} MB\``,
                    `**Image Size**: \`${(response.image.length / 1000 / 1000).toFixed(1)} MB\``
                ].join('\n');
            }
            if (response.image.length > guildAttachmentLimitBytes || args.upload) {
                output.content += '\n' + (yield this.uploadFile(response.image, `image/${response.format}`));
            }
            else {
                output = Object.assign(Object.assign({}, output), { file: {
                        filename: 'imagescript.' + response.format,
                        value: response.image
                    } });
            }
            return context.editOrReply(output);
        });
    }
}
exports.default = ImageScriptTagCommand;
