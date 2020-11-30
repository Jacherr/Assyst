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
class UptimeCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['up'];
        this.name = 'uptime';
        this.metadata = {
            description: 'Get the uptime of the Assyst process'
        };
    }
    run(context) {
        return __awaiter(this, void 0, void 0, function* () {
            const uptimeMillis = process.uptime() * 1000; // ms
            const uptime = utils_1.elapsed(uptimeMillis);
            return context.editOrReply(`Uptime: ${uptime.days}d ${uptime.hours}h ${uptime.minutes}m ${uptime.seconds}s`);
        });
    }
}
exports.default = UptimeCommand;
