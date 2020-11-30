"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Channels = void 0;
const basecollection_1 = require("./basecollection");
;
/**
 * Channels Collection
 * @category Collections
 */
class Channels extends basecollection_1.BaseClientCollection {
    insert(channel) {
        if (this.enabled) {
            this.set(channel.id, channel);
        }
    }
    get [Symbol.toStringTag]() {
        return `Channels (${this.size.toLocaleString()} items)`;
    }
}
exports.Channels = Channels;
