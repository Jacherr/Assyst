"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Profile = void 0;
const basecollection_1 = require("../collections/basecollection");
const baseset_1 = require("../collections/baseset");
const constants_1 = require("../constants");
const basestructure_1 = require("./basestructure");
const connectedaccount_1 = require("./connectedaccount");
const user_1 = require("./user");
const keysProfile = new baseset_1.BaseSet([
    constants_1.DiscordKeys.CONNECTED_ACCOUNTS,
    constants_1.DiscordKeys.MUTUAL_GUILDS,
    constants_1.DiscordKeys.PREMIUM_GUILD_SINCE,
    constants_1.DiscordKeys.PREMIUM_SINCE,
    constants_1.DiscordKeys.USER,
]);
/**
 * User Profile Structure
 * only non-bots will ever see these
 * @category Structure
 */
class Profile extends basestructure_1.BaseStructure {
    constructor(client, data) {
        super(client);
        this._keys = keysProfile;
        this.connectedAccounts = new basecollection_1.BaseCollection();
        this.mutualGuilds = new basecollection_1.BaseCollection();
        this.nicks = new basecollection_1.BaseCollection();
        this.premiumGuildSinceUnix = 0;
        this.premiumSinceUnix = 0;
        this.merge(data);
    }
    get premiumGuildSince() {
        if (this.premiumGuildSinceUnix) {
            return new Date(this.premiumGuildSinceUnix);
        }
        return null;
    }
    get premiumSince() {
        if (this.premiumSinceUnix) {
            return new Date(this.premiumSinceUnix);
        }
        return null;
    }
    mergeValue(key, value) {
        if (value !== undefined) {
            switch (key) {
                case constants_1.DiscordKeys.CONNECTED_ACCOUNTS:
                    {
                        this.connectedAccounts.clear();
                        for (let raw of value) {
                            const account = new connectedaccount_1.ConnectedAccount(this.client, raw);
                            this.connectedAccounts.set(account.key, account);
                        }
                    }
                    ;
                    return;
                case constants_1.DiscordKeys.MUTUAL_GUILDS:
                    {
                        this.mutualGuilds.clear();
                        for (let raw of value) {
                            this.mutualGuilds.set(raw.id, raw);
                        }
                    }
                    ;
                    return;
                case constants_1.DiscordKeys.PREMIUM_GUILD_SINCE:
                    {
                        this.premiumGuildSinceUnix = (value) ? (new Date(value)).getTime() : 0;
                    }
                    ;
                    return;
                case constants_1.DiscordKeys.PREMIUM_SINCE:
                    {
                        this.premiumSinceUnix = (value) ? (new Date(value)).getTime() : 0;
                    }
                    ;
                    return;
                case constants_1.DiscordKeys.USER:
                    {
                        value = new user_1.UserWithFlags(this.client, value);
                    }
                    ;
                    break;
            }
            return super.mergeValue(key, value);
        }
    }
}
exports.Profile = Profile;
