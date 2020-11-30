const BasePaginator = require("./BasePaginator");

module.exports = class ReactionPaginator extends BasePaginator {
    constructor(client, data) {
        super(client, data);
        this.waitingForPage = null;
        this.reactions = data.reactions || {
            firstPage: "⏮️",
            previousPage: "⬅️",
            nextPage: "➡️",
            lastPage: "⏭️",
            skipTo: "🔢",
            stop: "⏹️"
        };
    }

    async addReactions() {
        if (!this.commandMessage) return;

        for (const reactions of Object.values(this.reactions)) {
            if (this.isShared) {
                for (const msg of this.commandMessage.values()) {
                    await msg.react(reactions).catch();
                }                
            } else {
                await this.commandMessage.react(reactions).catch(() => {});
            }
        }
    }

    // TODO: this only works if cache is enabled
    // perhaps add option to use REST API to fetch all reactions?
    async clearReactions() {
        const reactions = this.isShared ? Array.from(this.commandMessage.values()).map(x => Array.from(x.reactions.values())).flat() : this.commandMessage.reactions.values();

        for (const reaction of reactions) {
            this.clearReaction(reaction.emoji.name);
        }
    }

    async clearReaction(emoji) {
        const reaction = this.commandMessage.reactions.find(x => x.emoji.name === emoji);

        if (reaction) {
            reaction.delete(this.message.author.id).catch(() => {});
        }
    }
};
