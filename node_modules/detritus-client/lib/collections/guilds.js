"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Guilds = void 0;
const basecollection_1 = require("./basecollection");
;
/**
 * Guilds Collection
 * @category Collections
 */
class Guilds extends basecollection_1.BaseClientCollection {
    insert(guild) {
        if (this.enabled) {
            this.set(guild.id, guild);
        }
    }
    get [Symbol.toStringTag]() {
        return `Guilds (${this.size.toLocaleString()} items)`;
    }
}
exports.Guilds = Guilds;
