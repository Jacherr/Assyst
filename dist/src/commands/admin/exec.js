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
class ExecCommand extends baseadmincommand_1.BaseAdminCommand {
    constructor() {
        super(...arguments);
        this.name = 'ex';
        this.label = 'script';
        this.metadata = {
            description: 'Execute shell commands',
            examples: ['ps aux'],
            usage: '[script]'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            yield context.triggerTyping();
            const result = yield this.exec(args.script);
            const output = utils_1.Markup.codeblock(result, { language: 'sh' });
            return context.editOrReply(output);
        });
    }
    exec(script) {
        return __awaiter(this, void 0, void 0, function* () {
            return Promise.race([
                new Promise((resolve, reject) => {
                    child_process_1.exec(script, (error, stdout, stderr) => {
                        if (stderr && !stdout)
                            reject(new Error(stderr));
                        if (error)
                            reject(new Error(error.message));
                        resolve(stdout);
                    });
                }),
                new Promise((_, r) => setTimeout(() => r('Timed out after 10000ms'), 10000))
            ]);
        });
    }
}
exports.default = ExecCommand;
