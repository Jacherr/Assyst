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
const constants_1 = require("detritus-client-socket/lib/constants");
const constants_2 = require("detritus-client/lib/constants");
const assyst_1 = require("./assyst");
const config_json_1 = require("../config.json");
const bot = new assyst_1.Assyst('', {
    activateOnEdits: true,
    cache: {
        channels: {
            enabled: false
        },
        members: {
            enabled: false
        },
        messages: {
            limit: 1000,
            expire: 60000
        },
        users: {
            enabled: false
        }
    },
    directory: './commands',
    gateway: {
        compress: false,
        identifyProperties: {
            $browser: 'Discord iOS'
        },
        intents: [
            constants_1.GatewayIntents.GUILD_MESSAGES,
            constants_1.GatewayIntents.GUILD_MESSAGE_REACTIONS
        ],
        presence: {
            activity: {
                name: 'discord.gg/uwRrTfJ',
                type: constants_2.ActivityTypes.WATCHING
            },
            status: constants_2.PresenceStatuses.ONLINE
        }
    },
    mentionsEnabled: true,
    ratelimits: [
        {
            duration: 60000,
            limit: 50,
            type: constants_2.CommandRatelimitTypes.GUILD
        },
        {
            duration: 10000,
            limit: 10,
            type: constants_2.CommandRatelimitTypes.CHANNEL
        }
    ]
});
bot.on(constants_2.ClientEvents.COMMAND_RATELIMIT, ({ command, context, global, ratelimits }) => __awaiter(void 0, void 0, void 0, function* () {
    let replied = false;
    for (const { item, ratelimit, remaining } of ratelimits) {
        if (remaining < 1000 || replied || item.replied) {
            item.replied = true;
            continue;
        }
        replied = item.replied = true;
        let content;
        if (global) {
            content = `This ${ratelimit.type} is on cooldown for ${(remaining / 1000).toFixed(1)} seconds.`;
        }
        else {
            content = `${command.name} is on cooldown in this ${ratelimit.type} for ${(remaining / 1000).toFixed(1)} seconds.`;
        }
        yield context.editOrReply(content);
    }
}));
bot.on('commandDelete', ({ reply }) => {
    reply.delete();
});
(() => __awaiter(void 0, void 0, void 0, function* () {
    var _a;
    const cluster = bot.client;
    process.title = `Assyst Cluster ${(_a = cluster.manager) === null || _a === void 0 ? void 0 : _a.clusterId}, Shards ${cluster.shardStart}-${cluster.shardEnd}`;
    cluster.on(constants_2.ClientEvents.SHARD, ({ shard }) => {
        shard.gateway.on(constants_1.SocketEvents.STATE, ({ state }) => __awaiter(void 0, void 0, void 0, function* () {
            yield bot.executeLogWebhook(config_json_1.logWebhooks.shards, `🌐 Shard \`${shard.shardId}\` is \`${state}\``);
        }));
    });
    yield bot.run();
    yield bot.executeLogWebhook(config_json_1.logWebhooks.shards, `🆗 Shards #(${cluster.shards.map((shard) => shard.shardId).join(', ')}) loaded`);
}))();
