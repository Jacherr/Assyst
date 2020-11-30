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
class InviteCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['inv'];
        this.name = 'invite';
        this.metadata = {
            description: 'Get the Assyst invite'
        };
    }
    run(context) {
        var _a;
        return __awaiter(this, void 0, void 0, function* () {
            const invite = (_a = context.client.application) === null || _a === void 0 ? void 0 : _a.oauth2UrlFormat({
                scope: 'bot'
            });
            context.editOrReply(`ℹ️ Invite Assyst with this URL: <${invite}>`);
        });
    }
}
exports.default = InviteCommand;
