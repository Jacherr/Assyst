"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Context = void 0;
/**
 * Command Context
 * @category Command
 */
class Context {
    constructor(message, typing, commandClient) {
        this.message = message;
        this.typing = typing;
        this.commandClient = commandClient;
        this.client = message.client;
        Object.defineProperties(this, {
            client: { enumerable: false, writable: false },
            command: { enumerable: false, writable: true },
            commandClient: { enumerable: false, writable: false },
            message: { writable: false },
        });
    }
    /* Generic Client Properties */
    get application() {
        return this.client.application;
    }
    get cluster() {
        return this.client.cluster;
    }
    get gateway() {
        return this.client.gateway;
    }
    get manager() {
        return (this.cluster) ? this.cluster.manager : null;
    }
    get owners() {
        return this.client.owners;
    }
    get rest() {
        return this.client.rest;
    }
    get shardCount() {
        return this.client.shardCount;
    }
    get shardId() {
        return this.client.shardId;
    }
    get response() {
        if (this.commandClient.replies.has(this.messageId)) {
            const { reply } = this.commandClient.replies.get(this.messageId);
            return reply;
        }
        return null;
    }
    /* Client Collections */
    get applications() {
        return this.client.applications;
    }
    get channels() {
        return this.client.channels;
    }
    get emojis() {
        return this.client.emojis;
    }
    get guilds() {
        return this.client.guilds;
    }
    get members() {
        return this.client.members;
    }
    get messages() {
        return this.client.messages;
    }
    get notes() {
        return this.client.notes;
    }
    get presences() {
        return this.client.presences;
    }
    get relationships() {
        return this.client.relationships;
    }
    get roles() {
        return this.client.roles;
    }
    get sessions() {
        return this.client.sessions;
    }
    get typings() {
        return this.client.typings;
    }
    get users() {
        return this.client.users;
    }
    get voiceCalls() {
        return this.client.voiceCalls;
    }
    get voiceConnections() {
        return this.client.voiceConnections;
    }
    get voiceStates() {
        return this.client.voiceStates;
    }
    /* Message Properties */
    get canDelete() {
        return this.message.canDelete;
    }
    get canManage() {
        return this.message.canManage;
    }
    get canReact() {
        return this.message.canReact;
    }
    get canReply() {
        return this.message.canReply;
    }
    get channel() {
        return this.message.channel;
    }
    get channelId() {
        return this.message.channelId;
    }
    get content() {
        return this.message.content;
    }
    get fromBot() {
        return this.message.fromBot;
    }
    get fromSystem() {
        return this.message.fromSystem;
    }
    get fromUser() {
        return this.message.fromUser;
    }
    get fromWebhook() {
        return this.message.fromWebhook;
    }
    get guild() {
        return this.message.guild;
    }
    get guildId() {
        return this.message.guildId;
    }
    get inDm() {
        return this.message.inDm;
    }
    get me() {
        const guild = this.guild;
        if (guild) {
            return guild.me;
        }
        return null;
    }
    get member() {
        return this.message.member;
    }
    get messageId() {
        return this.message.id;
    }
    get systemContent() {
        return this.message.systemContent;
    }
    get user() {
        return this.message.author;
    }
    get userId() {
        return this.message.author.id;
    }
    get voiceChannel() {
        const member = this.member;
        if (member) {
            return member.voiceChannel;
        }
        return null;
    }
    get voiceConnection() {
        return this.voiceConnections.get(this.guildId || this.channelId);
    }
    get voiceState() {
        const member = this.member;
        if (member) {
            return member.voiceState;
        }
        return null;
    }
    async editOrReply(options = {}) {
        if (typeof (options) === 'string') {
            options = { content: options };
        }
        let reply;
        if (this.commandClient.replies.has(this.messageId)) {
            options = Object.assign({ content: '', embed: null }, options);
            const old = this.commandClient.replies.get(this.messageId);
            if (old.reply.hasAttachment || options.activity || options.applicationId || options.file || options.files) {
                if (options.delete || options.delete === undefined) {
                    await old.reply.delete();
                }
                reply = await this.message.reply(options);
            }
            else {
                reply = await old.reply.edit(options);
            }
        }
        else {
            reply = await this.message.reply(options);
        }
        if (this.command) {
            this.commandClient.storeReply(this.messageId, this.command, this, reply);
        }
        return reply;
    }
    reply(options = {}) {
        return this.message.reply(options);
    }
    triggerTyping() {
        return this.message.triggerTyping();
    }
    toJSON() {
        return this.message;
    }
    toString() {
        return `Context (${this.messageId})`;
    }
}
exports.Context = Context;
