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
class PrefixCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.label = 'prefix';
        this.name = 'prefix';
        this.metadata = {
            description: 'View or set this guild\'s prefix',
            examples: ['', '-'],
            usage: '<new prefix>'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            if (args.prefix) {
                const isGuildOwner = yield this.userOwnsGuild(context);
                if (!isGuildOwner) {
                    return this.error(context, 'You need to own the guild to change the prefix.');
                }
                else if (args.prefix.length > 16) {
                    return this.error(context, 'The new prefix needs to be less than 16 characters.');
                }
                yield this.assyst.database.editGuildPrefix(context.guildId, args.prefix);
                return context.editOrReply(`Prefix changed to \`${args.prefix}\``);
            }
            else {
                const prefix = yield this.assyst.database.fetchGuildPrefix(context.guildId);
                return context.editOrReply(`Guild prefix: \`${prefix || 'None'}\``);
            }
        });
    }
}
exports.default = PrefixCommand;
