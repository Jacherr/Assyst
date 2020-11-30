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
class MaryjaneBansCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['mj bans'];
        this.name = 'maryjane bans';
        this.label = 'guildId';
        this.metadata = {
            description: 'Get bans for a guild from Maryjane API',
            examples: ['178313653177548800'],
            usage: '[guild id]'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            const guild = yield this.assyst.maryjane.guild(args.guildId || context.guildId);
            const bans = guild.bans;
            let output;
            if (bans.length === 0) {
                return this.error(context, 'This guild has no bans.');
            }
            else {
                let table = utils_1.generateTable({
                    offset: 4,
                    header: ['#', 'User ID', 'Reason'],
                    rows: bans.map((b, i) => { var _a; return [i, b.userid, (_a = b.reason) !== null && _a !== void 0 ? _a : 'N/A']; }).slice(0, 15)
                });
                output = utils_2.Markup.codeblock(table, { language: 'md' });
            }
            return context.editOrReply(output);
        });
    }
}
exports.default = MaryjaneBansCommand;
