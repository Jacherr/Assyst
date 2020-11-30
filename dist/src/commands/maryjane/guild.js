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
const utils_1 = require("../../utils");
const utils_2 = require("detritus-client/lib/utils");
class MaryjaneGuildCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['mj guild'];
        this.name = 'maryjane guild';
        this.label = 'guildId';
        this.metadata = {
            description: 'Get a guild from Maryjane API',
            examples: ['178313653177548800'],
            usage: '[guild id]'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            const guild = yield this.assyst.maryjane.guild(args.guildId || context.guildId);
            const kvList = utils_1.generateKVList([
                ['ID', guild.id],
                ['Name', guild.name],
                ['Description', guild.description !== null ? utils_1.flat(guild.description.split(' '), 5).map(a => a.join(' ')).join('\n') : 'N/A'],
                ['Members', String(guild.member_count)],
                ['Channels', String(guild.channel_count)],
                ['Emoji', String(guild.emoji_count)],
                ['Roles', String(guild.role_count)],
                ['Owner', guild.ownerid],
                ['Locale', guild.preferred_locale],
                ['Region', guild.region],
                ['Flags', String(guild.flags)],
                ['Invites', guild.invites.length > 0 ? guild.invites.map(i => i.invite).join(', ') : 'None'],
                ['Bans', String(guild.bans.length)]
            ]);
            return context.editOrReply(utils_2.Markup.codeblock(kvList, { language: 'ml' }));
        });
    }
}
exports.default = MaryjaneGuildCommand;
