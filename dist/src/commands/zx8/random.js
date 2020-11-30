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
const basecommand_1 = require("../basecommand");
class Zx8RandomCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.args = [
            {
                name: 'bomb',
                type: Boolean,
                default: false
            }
        ];
        this.name = 'zx8 random';
        this.label = 'type';
        this.metadata = {
            description: 'Get random zx8 thing',
            examples: ['image'],
            usage: '[image|html|video|audio]'
        };
    }
    run(context, args) {
        var _a;
        return __awaiter(this, void 0, void 0, function* () {
            yield context.triggerTyping();
            let data;
            let ext;
            switch (args.type.toLowerCase()) {
                case 'audio': {
                    data = yield this.assyst.zx8.randomAudio();
                    ext = 'mp3';
                    break;
                }
                case 'video': {
                    data = yield this.assyst.zx8.randomVideo();
                    ext = 'mp4';
                    break;
                }
                case 'image': {
                    data = yield this.assyst.zx8.randomImage();
                    ext = 'png';
                    break;
                }
                case 'html': {
                    data = yield this.assyst.zx8.randomHtml();
                    const site = data[0];
                    let screenshot;
                    try {
                        screenshot = yield this.assyst.screenshot(site.url, (_a = context.channel) === null || _a === void 0 ? void 0 : _a.nsfw, 0);
                    }
                    catch (e) {
                        return this.error(context, `${site.url}\n\n${e.message}`);
                    }
                    return context.editOrReply({
                        content: `<${site.url}>`,
                        file: {
                            filename: 'zx8.png',
                            value: screenshot
                        }
                    });
                }
                default: {
                    return this.error(context, 'Specify image, video, html or audio.');
                }
            }
            const urls = [];
            const limit = (args.bomb ? 5 : 1);
            const promises = [];
            for (let i = 0; i < limit; i++) {
                if (data[i]) {
                    urls.push(`<${data[i].url}>`);
                    promises.push(context.rest.request({ url: data[i].url, timeout: 5000 }));
                }
            }
            let files;
            try {
                files = (yield Promise.all(promises))
                    .map((x, i) => ({
                    filename: `zx8_${i}.${ext}`,
                    value: x
                }));
            }
            catch (e) {
                return this.error(context, `${urls.join('\n')}\n\n${e.message}`);
            }
            try {
                yield context.reply({
                    content: urls.join('\n'),
                    files
                });
            }
            catch (e) {
                return this.error(context, `${urls.join('\n')}\n\n${e.message}`);
            }
        });
    }
}
exports.default = Zx8RandomCommand;
