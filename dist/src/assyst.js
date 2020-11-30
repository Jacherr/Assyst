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
const detritus_client_1 = require("detritus-client");
const fapi_client_1 = require("fapi-client");
const detritus_pagination_1 = require("detritus-pagination");
const config_json_1 = require("../config.json");
const zx8_1 = require("./rest/zx8/zx8");
const maryjane_1 = require("./rest/maryjane/maryjane");
const database_1 = require("./database/database");
const crypto_1 = require("crypto");
const errorImages = {
    fcab5a15e2ee436f8694b9777c3cb08b: 'No DNS records.',
    '5991482f1a1d321eea4162044abbfd78': 'The domain does not exist.',
    d4e18d5b499eedb1b3b62d93a669beb8: 'Connection refused.',
    ab341be5ab990e3179bb6c4db954f702: 'Connection reset by peer.'
};
class Assyst extends detritus_client_1.CommandClient {
    constructor(token, options) {
        super(token, options);
        this.database = new database_1.Database(this, config_json_1.database);
        this.directory = options.directory;
        this.fapi = new fapi_client_1.Client.Client({
            auth: config_json_1.tokens.fapi,
            timeout: 45000
        });
        this.paginator = new detritus_pagination_1.PaginatorCluster(this.client, {
            maxTime: 60000,
            pageNumber: true
        });
        this.maryjane = new maryjane_1.Maryjane();
        this.zx8 = new zx8_1.Zx8();
    }
    executeLogWebhook(url, options) {
        const searchString = 'webhooks';
        const index = url.indexOf(searchString);
        if (index === -1) {
            throw new Error('Invalid Discord webhook URL provided');
        }
        const id = url.slice(index + searchString.length + 1, url.lastIndexOf('/'));
        const token = url.slice(url.lastIndexOf('/') + 1);
        // @ts-ignore
        return this.rest.executeWebhook(id, token, options);
    }
    resetCommands() {
        return __awaiter(this, void 0, void 0, function* () {
            this.clear();
            yield this.addMultipleIn(this.directory, {
                subdirectories: true
            });
        });
    }
    run(options) {
        const _super = Object.create(null, {
            run: { get: () => super.run }
        });
        return __awaiter(this, void 0, void 0, function* () {
            yield this.resetCommands();
            return _super.run.call(this, options);
        });
    }
    onCommandCheck(context, command) {
        return __awaiter(this, void 0, void 0, function* () {
            const whitelist = config_json_1.channelWhitelist;
            const whitelistVerifiedAllowed = whitelist.length > 0 ? whitelist.includes(context.channelId) : true;
            if (context.user.isClientOwner) {
                return true;
            }
            else if (context.inDm ||
                context.user.bot ||
                !whitelistVerifiedAllowed ||
                (config_json_1.ownerOnly && (!context.user.isClientOwner && !config_json_1.admins.includes(context.userId)))) {
                return false;
            }
            return true;
        });
    }
    onPrefixCheck(context) {
        return __awaiter(this, void 0, void 0, function* () {
            if (config_json_1.prefixOverride)
                return config_json_1.prefixOverride;
            const defaultPrefix = 'a-';
            const userId = context.client.userId;
            const prefixes = new Set([`<@${userId}>`, `<@!${userId}>`]);
            const customPrefix = yield this.database.fetchGuildPrefix(context.guildId);
            if (!customPrefix) {
                yield this.database.setGuildPrefix(context.guildId, defaultPrefix);
                prefixes.add(defaultPrefix);
                return prefixes;
            }
            prefixes.add(customPrefix);
            return prefixes;
        });
    }
    screenshot(url, allowNsfw, wait) {
        return __awaiter(this, void 0, void 0, function* () {
            const res = yield this.fapi.screenshot(url, {
                allowNSFW: allowNsfw,
                wait
            });
            const hash = crypto_1.createHash('md5').update(res).digest('hex');
            if (errorImages[hash] !== undefined)
                throw new Error(errorImages[hash]);
            return res;
        });
    }
}
exports.Assyst = Assyst;
