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
const child_process_1 = require("child_process");
const baseadmincommand_1 = require("../baseadmincommand");
const utils_1 = require("detritus-client/lib/utils");
class UpdateCommand extends baseadmincommand_1.BaseAdminCommand {
    constructor() {
        super(...arguments);
        this.name = 'update';
        this.metadata = {
            description: 'Update the bot',
            examples: [''],
            usage: ''
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            yield context.triggerTyping();
            const out = child_process_1.execSync('git pull && tsc');
            yield context.reply(utils_1.Markup.codeblock(out.toString(), {
                language: 'bash'
            }));
            delete require.cache;
            this.assyst.resetCommands();
        });
    }
}
exports.default = UpdateCommand;
