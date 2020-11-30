"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Members = void 0;
const basecollection_1 = require("./basecollection");
const constants_1 = require("../constants");
;
/**
 * Members Collection
 * @category Collections
 */
class Members extends basecollection_1.BaseClientGuildReferenceCache {
    constructor() {
        super(...arguments);
        this.key = constants_1.DetritusKeys[constants_1.DiscordKeys.MEMBERS];
    }
    insert(member) {
        const guild = member.guild;
        if (guild) {
            if (member.isMe) {
                guild.members.set(member.id, member);
            }
            else {
                if (this.enabled) {
                    guild.members.set(member.id, member);
                }
            }
        }
    }
    get [Symbol.toStringTag]() {
        return `Members (${this.size.toLocaleString()} items)`;
    }
}
exports.Members = Members;
