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
const CREPLS_ID = '302394290368151553';
class CreplsCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.name = 'crepls';
        this.metadata = {
            description: 'Get crepls'
        };
    }
    run(context) {
        return __awaiter(this, void 0, void 0, function* () {
            const crepls = yield context.rest.fetchUser(CREPLS_ID);
            const creplsAvatar = crepls.avatarUrl;
            const buffer = yield node_fetch_1.default(creplsAvatar).then(x => x.buffer());
            return context.editOrReply({
                content: 'this is crepls',
                file: {
                    filename: 'crepls.png',
                    value: buffer
                }
            });
        });
    }
}
exports.default = CreplsCommand;
