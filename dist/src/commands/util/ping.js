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
class PingCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['pong'];
        this.label = 'host';
        this.name = 'ping';
        this.metadata = {
            description: 'Ping the Discord REST and WebSocket APIs'
        };
    }
    run(context, _args) {
        return __awaiter(this, void 0, void 0, function* () {
            const { gateway, rest } = yield context.client.ping();
            return context.editOrReply(`Pong! REST: ${rest}ms, WS: ${gateway}ms`);
        });
    }
}
exports.default = PingCommand;
