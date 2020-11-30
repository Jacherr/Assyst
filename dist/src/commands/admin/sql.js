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
const utils_1 = require("detritus-client/lib/utils");
const util_1 = require("util");
const baseadmincommand_1 = require("../baseadmincommand");
class SQLCommand extends baseadmincommand_1.BaseAdminCommand {
    constructor() {
        super(...arguments);
        this.label = 'query';
        this.name = 'sql';
        this.metadata = {
            description: 'Execute SQL',
            examples: ['select now(*)'],
            usage: '[query]'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            const result = yield this.assyst.database.sql(args.query).then(res => res.rows);
            return context.editOrReply(utils_1.Markup.codeblock(util_1.inspect(result), {
                language: 'js',
                limit: 1990
            }));
        });
    }
}
exports.default = SQLCommand;
