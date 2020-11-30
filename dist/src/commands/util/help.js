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
const constants_1 = require("../../constants");
const utils_1 = require("detritus-client/lib/utils");
class HelpCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.label = 'command';
        this.name = 'help';
        this.metadata = {
            description: 'Get Assyst full or specific command help',
            examples: ['', 'ping'],
            usage: '<command>'
        };
    }
    run(context, args) {
        var _a;
        return __awaiter(this, void 0, void 0, function* () {
            if (!args.command) {
                const categories = [];
                for (const command of this.assyst.commands) {
                    if ((yield ((_a = command.onBefore) === null || _a === void 0 ? void 0 : _a.call(command, context))) === true) {
                        const categoryNames = categories.map(c => c.name);
                        const commandCategory = this.getCategory(command._file);
                        if (!categoryNames.includes(commandCategory)) {
                            categories.push({
                                name: commandCategory,
                                commands: [command.name]
                            });
                        }
                        else {
                            const category = categories.find(c => c.name === commandCategory);
                            category.commands.push(command.name);
                        }
                    }
                }
                const fields = categories.map(c => ({
                    name: c.name,
                    value: c.commands.map(cmd => `\`${cmd}\``).join(','),
                    inline: false
                }));
                return context.editOrReply({
                    embed: {
                        fields,
                        color: constants_1.EmbedColors.INFO,
                        title: 'Assyst Commands'
                    }
                });
            }
            else {
                const command = this.assyst.commands.find((c) => {
                    return c.name === args.command || c.aliases.includes(args.command);
                });
                if (!command) {
                    return this.error(context, 'No command with this name or alias exists.');
                }
                return context.editOrReply({
                    embed: {
                        color: constants_1.EmbedColors.INFO,
                        description: command.metadata.description || 'No description found...',
                        title: `Command: ${command.name}`,
                        fields: [
                            {
                                name: 'Usage',
                                value: (() => {
                                    const usage = command.metadata.usage;
                                    if (!usage)
                                        return utils_1.Markup.codeblock(context.prefix + command.name);
                                    return utils_1.Markup.codeblock(context.prefix + command.name + ' ' + usage);
                                })(),
                                inline: false
                            },
                            {
                                name: 'Examples',
                                value: (() => {
                                    const examples = command.metadata.examples;
                                    if (!examples)
                                        return utils_1.Markup.codeblock(context.prefix + command.name);
                                    return utils_1.Markup.codeblock(examples.map((e) => `${context.prefix}${command.name} ${e}`).join('\n'));
                                })()
                            }
                        ]
                    }
                });
            }
        });
    }
    getCategory(path) {
        const parts = path.replace(/\\/g, '/').split('/');
        const category = parts[parts.length - 2];
        return category;
    }
}
exports.default = HelpCommand;
