const EventEmitter = require("eventemitter3");
const {Context} = require("detritus-client/lib/command");

module.exports = class BasePaginator extends EventEmitter {
    constructor(client, data) {
        super();
        this.client = client;
        this.message = BasePaginator.asMessage(data.message);
        this.commandMessage = data.commandMessage || null;
        this.pages = data.pages;
        this.index = 0;
        this.targetUser = data.targetUser || this.message.author.id;

        // Reference to reply function
        // Uses context.editOrReply if an instance of Context was passed
        // Defaults to message.reply
        this.editOrReply = (data.message.editOrReply || data.message.reply).bind(data.message);
    }

    static asMessage(ctx) {
        return ctx instanceof Context ? ctx.message : ctx;
    }

    get isShared() {
        return this.commandMessage instanceof Map;
    }

    isCommandMessage(messageId) {
        if (!this.commandMessage) return false;

        return this.isShared ? this.commandMessage.has(messageId) : this.commandMessage.id === messageId;
    }

    isInChannel(channelId) {
        if (!this.commandMessage) return false;

        return this.isShared ? Array.from(this.commandMessage.values()).some(x => x.channelId === channelId) : this.commandMessage.channelId === channelId;
    }

    isTarget(user) {
        return this.targetUser instanceof Set ? this.targetUser.has(user) : this.targetUser === user;
    }

    async update(data) {
        if (this.isShared) {
            for (const m of this.commandMessage.values()) {
                await m.edit(data);
            }
        } else if (this.commandMessage) {
            this.commandMessage.edit(data);
        }
    }

    async init() {
        return this.commandMessage = await this.editOrReply(this.pages[this.index]);
    }

    async previous() {
        if (Array.isArray(this.pages) && this.pages.length > 0) {
            if (this.client.pageLoop) {
                await this.update(this.pages[this.index === 0 ? this.index = this.pages.length - 1 : --this.index]);
            } else if (this.index !== 0) {
                await this.update(this.pages[--this.index]);
            } else {
                return this.commandMessage;
            }
        }
        this.emit("previous", this);
        return this.commandMessage;
    }

    async next() {
        if (Array.isArray(this.pages) && this.pages.length > 0) {
            if (this.client.pageLoop) {
                await this.update(this.pages[this.index === this.pages.length - 1 ? this.index = 0 : ++this.index]);
            } else if (this.index !== this.pages.length -1) {
                await this.update(this.pages[++this.index]);
            } else {
                return this.commandMessage;
            }
        }
        this.emit("next", this);
        return this.commandMessage;
    }

    async jumpTo(page) {
        if (isNaN(page) || this.pages[page] === undefined) {
            throw new Error("Invalid page");
        }
        await this.update(this.pages[page]);

        this.emit("page", {
            page,
            paginator: this
        });
        return this.commandMessage;
    }

    stop(timeout = false) {
        this.emit("stop", this, timeout);
        this.removeAllListeners();
        const targetIndex = this.client.activeListeners.findIndex(v => v.message.id === this.message.id);
        this.client.activeListeners.splice(targetIndex, 1);
        return this;
    }
};