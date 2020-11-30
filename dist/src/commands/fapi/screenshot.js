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
class ScreenshotCommand extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.label = 'url';
        this.name = 'screenshot';
        this.aliases = ['ss'];
        this.args = [
            {
                name: 'wait',
                default: '0',
                type: Number
            }
        ];
        this.metadata = {
            description: 'Screenshot a webpage',
            examples: ['https://jacher.io/'],
            usage: '[url]'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            const c = yield context.rest.fetchChannel(context.channelId);
            const res = yield this.assyst.screenshot(args.url, c.nsfw, args.wait);
            return context.editOrReply({
                file: {
                    filename: 'screenshot.png',
                    value: res
                }
            });
        });
    }
}
exports.default = ScreenshotCommand;
