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
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const basecommand_1 = require("../basecommand");
const node_fetch_1 = __importDefault(require("node-fetch"));
class AvatarCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['av'];
        this.label = 'user';
        this.name = 'avatar';
        this.metadata = {
            description: 'Get a user\'s avatar'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            let user;
            if (args.user) {
                let id = this.parseMentionOrId(args.user.split(/\s/g)[0]);
                try {
                    user = yield context.rest.fetchUser(id);
                }
                catch (_a) {
                    return this.error(context, 'User not found');
                }
            }
            else {
                user = context.user;
            }
            let avatar = user.avatarUrl;
            let ext = avatar.split('.').pop();
            let buffer = yield node_fetch_1.default(avatar).then(x => x.buffer());
            return context.editOrReply({
                file: {
                    filename: `avatar.${ext}`,
                    value: buffer
                }
            });
        });
    }
}
exports.default = AvatarCommand;
