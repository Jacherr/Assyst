import { Assyst } from './assyst';
import { badTranslate } from './rest/rest';
import { Webhook, Message } from 'detritus-client/lib/structures';
import {
    TRANSLATION_RATELIMIT_RESET,
    MAX_CACHE_SIZE,
    MAX_MESSAGE_LENGTH,
    RATELIMIT_MESSAGE
} from './constants/badtranslator';
import * as Constants from './constants';

function isRatelimitMessage(message: Message) {
    return message.author.isMe && message.content.endsWith(RATELIMIT_MESSAGE);
}

export default class BadTranslator {
    private channels: Set<string>;
    private cachedWebhook: Map<string, Webhook>;
    private ratelimits: Map<string, number>; // maps user id to timestamp
    private bot: Assyst;

    constructor(bot: Assyst, channels: Array<string> | Set<string>) {
        this.bot = bot;
        this.cachedWebhook = new Map();
        this.ratelimits = new Map();
        this.channels = Array.isArray(channels)
            ? new Set(channels)
            : channels;
    }

    async transformContent(message: Message) {
        const emojiReplacedContent = message.content.replace(Constants.EMOJI_REGEX, (_, name) => name);

        const userIds = Array.from(emojiReplacedContent.matchAll(Constants.USER_ID))
            .map(x => x[1]);

        const users = await this.bot.maryjane.bulkUser(userIds)
            .catch(() => []);

        return emojiReplacedContent.replace(Constants.USER_ID, (match, id) => {
            return users.find(x => x.id === id)?.username || match;
        });
    }

    async init() {
        this.bot.client.on('messageCreate', async ({message}) => {
            if (!this.channels.has(message.channelId) || message.author.isWebhook || isRatelimitMessage(message)) return;
            if (message.content.length === 0 || message.content.length > MAX_MESSAGE_LENGTH || message.author.bot) return message.delete();

            const isRatelimited = this.isRatelimited(message.author.id);
            if (isRatelimited) {
                // If user is ratelimited, return early...
                message.reply(message.author.mention + RATELIMIT_MESSAGE).then(m => setTimeout(() => m.delete(), 5000));
                return message.delete();
            }

            const transformedContent = await this.transformContent(message);

            const translation = await badTranslate(transformedContent);

            await message.delete();

            // If the Bad Translator API returned an empty result
            // Return early
            if (!translation) return;

            const webhook = await this.getWebhook(message.channelId);

            // If the webhook wasn't found, return
            // TODO: Maybe add a console.warn or something, this can probably result
            // in hard to debug bugs
            if (!webhook || !webhook.token) return;

            //await webhook.execute({
            await this.bot.rest.executeWebhook(webhook.id, webhook.token, {
                allowedMentions: {
                    parse: []
                },
                avatarUrl: message.author.avatarUrl,
                content: translation,
                username: message.member?.nick || message.author.username || 'Bad Translator'
            });
        });
    }

    private isRatelimited(userId: string) {
        const ratelimit = this.ratelimits.get(userId);

        if (ratelimit) {
            if (Date.now() > ratelimit + TRANSLATION_RATELIMIT_RESET) {
                this.ratelimits.delete(userId);
                return false;
            }

            return true;
        }

        if (this.ratelimits.size > MAX_CACHE_SIZE) {
            // This might remove a ratelimit that was just added but it's fine
            // Ratelimits are only 5s so it doesn't matter if that happens
            this.ratelimits.delete(this.ratelimits.keys().next().value);
        }
        
        this.ratelimits.set(userId, Date.now());
    }

    private async getWebhook(channelId: string) {
        const cached = this.cachedWebhook.get(channelId);
        if (cached) return cached;

        const webhook: Webhook | undefined = await this.bot.rest.fetchChannelWebhooks(channelId)
            .then((webhooks) => webhooks[0]);
        
        if (webhook) {
            this.cachedWebhook.set(channelId, webhook);
            return webhook;
        }
    }
}