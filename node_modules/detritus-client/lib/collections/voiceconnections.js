"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.VoiceConnections = void 0;
const basecollection_1 = require("./basecollection");
;
/**
 * VoiceConnections Collection
 * @category Collections
 */
class VoiceConnections extends basecollection_1.BaseClientCollection {
    insert(connection) {
        if (this.enabled) {
            this.set(connection.serverId, connection);
        }
    }
    get [Symbol.toStringTag]() {
        return `VoiceConnections (${this.size.toLocaleString()} items)`;
    }
}
exports.VoiceConnections = VoiceConnections;
