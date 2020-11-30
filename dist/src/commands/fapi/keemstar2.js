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
class Keemstar2Command extends basefapicommand_1.BaseFapiCommand {
    constructor() {
        super(...arguments);
        this.label = 'url';
        this.name = 'keemstar2';
        this.metadata = {
            description: 'Keemstar 2',
            examples: ['https://link.to.my/image.png'],
            usage: '[url|attachment]'
        };
    }
    run(context, args) {
        return __awaiter(this, void 0, void 0, function* () {
            const url = yield this.getUrlFromChannel(context, args.url);
            if (!url) {
                return this.error(context, 'No valid URL was found... Please use an attachment or valid image URL');
            }
            const res = yield this.fapi.keemstar2(url);
            return context.editOrReply({
                file: {
                    filename: 'keemstar2.png',
                    value: res
                }
            });
        });
    }
}
exports.default = Keemstar2Command;
