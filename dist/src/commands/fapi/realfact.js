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
const basefapicommand_1 = require("../basefapicommand");
class RealFactCommand extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['fact'];
        this.label = 'text';
        this.name = 'realfact';
        this.metadata = {
            description: 'Real Fact',
            examples: [''],
            usage: '[text]'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            const res = yield this.fapi.realFact(args.text);
            return context.editOrReply({
                file: {
                    filename: 'realfact.png',
                    value: res
                }
            });
        });
    }
}
exports.default = RealFactCommand;
