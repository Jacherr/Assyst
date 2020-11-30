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
const basecommand_1 = require("./basecommand");
const utils_1 = require("../utils");
class BaseFapiCommand extends basecommand_1.BaseCommand {
    constructor(commandClient, options) {
        super(commandClient, Object.assign({
            name: '',
            ratelimits: [
                { duration: 5000, limit: 5, type: 'guild' },
                { duration: 2000, limit: 1, type: 'channel' }
            ]
        }, options));
    }
    get fapi() {
        return this.commandClient.fapi;
    }
    onBeforeRun(context) {
        return __awaiter(this, void 0, void 0, function* () {
            yield context.triggerTyping();
            return true;
        });
    }
    getRecentAttachmentOrEmbed(msg, amtOfMessages) {
        var _a;
        return __awaiter(this, void 0, void 0, function* () {
            if (msg.attachments.length > 0) {
                return (_a = msg.attachments.first()) === null || _a === void 0 ? void 0 : _a.url;
            }
            const messages = yield this.commandClient.rest.fetchMessages(msg.channelId, { limit: amtOfMessages });
            if (!messages) {
                return undefined;
            }
            let attachment;
            for (const message of messages) {
                if (message.attachments.length > 0) {
                    // types broke
                    // @ts-ignore
                    return message.attachments[0].url;
                }
                else if (message.embeds.length > 0) {
                    // types broke
                    // @ts-ignore
                    const embed = message.embeds[0];
                    if (embed === null || embed === void 0 ? void 0 : embed.thumbnail) {
                        return embed.thumbnail.url;
                    }
                    else if (embed === null || embed === void 0 ? void 0 : embed.image) {
                        return embed.image.url;
                    }
                    else {
                        continue;
                    }
                }
            }
            return attachment;
        });
    }
    getUrlFromChannel(ctx, args) {
        return __awaiter(this, void 0, void 0, function* () {
            let imageUrl;
            if (args) {
                imageUrl = args;
                try {
                    const parsedURL = new URL(imageUrl);
                    imageUrl = parsedURL.origin + parsedURL.pathname + parsedURL.search;
                }
                catch (e) {
                    return undefined;
                }
            }
            else {
                imageUrl = yield this.getRecentAttachmentOrEmbed(ctx.message, 50);
            }
            return imageUrl;
        });
    }
    parseImageScriptArgs(args) {
        const indexOfWhitespace = args.search(/\s/);
        if (indexOfWhitespace === -1)
            return [args, ''];
        const firstArg = args.slice(0, indexOfWhitespace);
        const restArgs = args.slice(indexOfWhitespace);
        return [firstArg, restArgs.trim()];
    }
    injectImageScriptPackages(script) {
        return __awaiter(this, void 0, void 0, function* () {
            const directive = '///USE';
            const lines = script.split('\n');
            let index = -1;
            const importedPackages = [];
            for (const line of lines) {
                index++;
                if (line.startsWith(directive)) {
                    const packageName = line.split(' ')[1].trim();
                    if (!packageName || importedPackages.includes(packageName))
                        continue;
                    const isPackage = yield this.assyst.database.fetchImageScriptPackage(packageName);
                    if (!isPackage)
                        continue;
                    lines[index] = `(() => { 
          try {
            ${isPackage.content} 
          } catch(_packageError) {
            throw new Error('Package \\'${isPackage.name}\\' threw an error: ' + (_packageError.message || _packageError))
          }
        })();`;
                    importedPackages.push(isPackage.name);
                }
            }
            return lines.join('\n');
        });
    }
    loadCode(context, messageContent) {
        return __awaiter(this, void 0, void 0, function* () {
            let code;
            if (context.message.attachments.first()) {
                const attachment = context.message.attachments.first();
                if (!attachment.url)
                    code = utils_1.parseCodeblocks(messageContent);
                const data = yield context.rest.request(attachment.url);
                code = data;
            }
            else {
                code = utils_1.parseCodeblocks(messageContent);
            }
            return code;
        });
    }
}
exports.BaseFapiCommand = BaseFapiCommand;
