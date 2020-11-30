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
const utils_2 = require("detritus-client/lib/utils");
const os_1 = require("os");
class InfoCommand extends basecommand_1.BaseCommand {
    constructor() {
        super(...arguments);
        this.aliases = ['stats'];
        this.label = 'host';
        this.name = 'info';
        this.metadata = {
            description: 'Get information about Assyst'
        };
    }
    run(context) {
        var _a, _b;
        return __awaiter(this, void 0, void 0, function* () {
            const result = utils_1.generateKVList([
                [
                    'Guilds',
                    (yield context.rest.fetchMeGuilds()).size.toString()
                ],
                [
                    'Clusters',
                    ((_a = context.manager) === null || _a === void 0 ? void 0 : _a.clusterCount.toString()) || '1'
                ],
                [
                    'Memory Usage',
                    yield ((_b = context.manager) === null || _b === void 0 ? void 0 : _b.broadcastEval('process.memoryUsage().rss / 1000 /1000').then((results) => {
                        return results.reduce((a, b) => a + b).toFixed(1) + 'MB';
                    }))
                ],
                [
                    'Commands',
                    this.assyst.commands.length.toString()
                ],
                [
                    'Authors',
                    'Jacher#9891, y21#0909'
                ],
                [
                    'Database Size',
                    yield this.assyst.database.getDatabaseSize()
                ],
                [
                    'CPU Model',
                    `${os_1.cpus().length}x ${os_1.cpus()[0].model}`
                ]
            ]);
            return context.editOrReply(utils_2.Markup.codeblock(result, {
                language: 'js'
            }));
        });
    }
}
exports.default = InfoCommand;
