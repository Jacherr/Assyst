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
const detritus_client_1 = require("detritus-client");
const config_json_1 = require("../../config.json");
const constants_1 = require("../constants");
const rest_1 = require("../rest/rest");
class BaseCommand extends detritus_client_1.Command.Command {
    constructor(commandClient, options) {
        super(commandClient, Object.assign({
            name: '',
            ratelimits: [
                { duration: 5000, limit: 5, type: 'guild' },
                { duration: 1000, limit: 1, type: 'channel' }
            ]
        }, options));
        this.responseOptional = true;
    }
    get assyst() {
        return this.commandClient;
    }
    error(context, content) {
        return __awaiter(this, void 0, void 0, function* () {
            return context.editOrReply({
                embed: {
                    color: constants_1.EmbedColors.ERROR,
                    title: '⚠️ Command Error',
                    description: content.slice(0, 1500)
                }
            });
        });
    }
    uploadFile(data, contentType) {
        return __awaiter(this, void 0, void 0, function* () {
            return rest_1.uploadToTsu(data, contentType);
        });
    }
    userOwnsGuild(context) {
        return __awaiter(this, void 0, void 0, function* () {
            const guild = yield context.rest.fetchGuild(context.guildId);
            return guild.ownerId === context.userId ||
                context.client.owners.map(u => u.id).includes(context.userId) ||
                config_json_1.admins.includes(context.userId);
        });
    }
    parseMentionOrId(input) {
        const match = input.match(/^<@!?(\d{17,19})>/);
        if (match) {
            return match[1];
        }
        else {
            return input;
        }
    }
    onBefore(context) {
        return __awaiter(this, void 0, void 0, function* () {
            const oldEditOrReply = context.editOrReply.bind(context);
            context.editOrReply = (options) => {
                if (typeof options === 'string') {
                    return oldEditOrReply({
                        content: options,
                        allowedMentions: {
                            parse: []
                        }
                    });
                }
                else {
                    return oldEditOrReply(Object.assign(Object.assign({}, options), { allowedMentions: {
                            parse: []
                        } }));
                }
            };
            return true;
        });
    }
    onRunError(context, _, error) {
        var _a;
        return __awaiter(this, void 0, void 0, function* () {
            const commandClient = context.commandClient;
            const description = [error.message || error.stack];
            if (error.response) {
                const response = error.response;
                try {
                    const information = yield response.json();
                    if ('errors' in information) {
                        for (const key in information.errors) {
                            const value = information.errors[key];
                            let message;
                            if (typeof (value) === 'object') {
                                message = JSON.stringify(value);
                            }
                            else {
                                message = String(value);
                            }
                            description.push(`**${key}**: ${message}`);
                        }
                    }
                }
                catch (e) {
                    description.push(yield response.text());
                }
            }
            yield commandClient.executeLogWebhook(config_json_1.logWebhooks.commandErrors, {
                embed: {
                    color: constants_1.EmbedColors.ERROR,
                    description: description.join('\n').slice(0, 1500),
                    fields: [
                        {
                            name: 'Command',
                            value: ((_a = context.command) === null || _a === void 0 ? void 0 : _a.name) || '',
                            inline: true
                        }
                    ],
                    title: '⚠️ Command Error'
                }
            });
            yield this.error(context, description.join('\n').slice(0, 1500));
        });
    }
    onTypeError(context, args, errors) {
        const store = {};
        const description = ['Invalid Arguments' + '\n'];
        for (const key in errors) {
            const message = errors[key].message;
            if (message in store) {
                description.push(`**${key}**: Same error as **${store[message]}**`);
            }
            else {
                description.push(`**${key}**: ${message}`);
            }
            store[message] = key;
        }
        return context.editOrReply({
            embed: {
                color: constants_1.EmbedColors.ERROR,
                description: description.join('\n').slice(0, 1500),
                title: '⚠️ Command Argument Error'
            }
        });
    }
}
exports.BaseCommand = BaseCommand;
