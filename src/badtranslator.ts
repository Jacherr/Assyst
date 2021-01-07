import { Assyst } from './assyst';
import { badTranslate } from './rest/rest';
import { Webhook } from 'detritus-client/lib/structures';

export default class BadTranslator {
    private channels: Set<string>;
    private cachedWebhook: Webhook | null;
    private bot: Assyst;

    constructor(bot: Assyst, channels: Array<string> | Set<string>) {
        this.bot = bot;
        this.cachedWebhook = null;
        this.channels = Array.isArray(channels)
            ? new Set(channels)
            : channels;
    }

    async init() {
        this.bot.client.on('messageCreate', async ({message}) => {
            if (!this.channels.has(message.channelId)) return;
            if (message.content.length === 0 || message.content.length > 500) return; // TODO: delete
            if (message.author.bot) return;

            const translation = await badTranslate(message.content);

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

    private async getWebhook(channelId: string) {
        if (this.cachedWebhook) return this.cachedWebhook;

        const webhook: Webhook | undefined = await this.bot.rest.fetchChannelWebhooks(channelId)
            .then((webhooks) => webhooks[0]);
        
        if (webhook) {
            return this.cachedWebhook = webhook;
        }
    }
}