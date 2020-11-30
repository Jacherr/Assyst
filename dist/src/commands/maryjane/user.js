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
class MaryjaneUserCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['mj user'];
        this.name = 'maryjane user';
        this.label = 'userId';
        this.metadata = {
            description: 'Get a user from Maryjane API',
            examples: ['571661221854707713'],
            usage: '[user id]'
        };
    }
    run(context, args) {
        var _a;
        return __awaiter(this, void 0, void 0, function* () {
            const user = yield this.assyst.maryjane.user(((_a = args.userId) === null || _a === void 0 ? void 0 : _a.replace(/[<>@!]/g, '')) || context.userId);
            const kvList = utils_1.generateKVList([
                ['ID', user.id],
                ['Tag', user.tag],
                ['Bot', String(user.bot)],
                ['Flags', String(user.flags)],
                ['Guilds', String(user.totalGuilds)],
                ['Bans', String(user.bans.length)],
                ['Connections', String(user.connections.length)],
                ['Premium_Since', user.premiumsince ? new Date(user.premiumsince).toLocaleString() : 'N/A']
            ]);
            return context.editOrReply(utils_2.Markup.codeblock(kvList, { language: 'ml' }));
        });
    }
}
exports.default = MaryjaneUserCommand;
