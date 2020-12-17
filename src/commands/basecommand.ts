import { Message } from 'detritus-client/lib/structures';
import { Command, CommandClient } from 'detritus-client';
import { Response } from 'detritus-rest';

import { Assyst } from '../assyst';

import { logWebhooks, admins } from '../../config.json';
import { EmbedColors } from '../constants';
import { uploadToTsu } from '../rest/rest';
import { Context } from 'detritus-client/lib/command';

export interface CommandMetadata {
  description?: string,
  examples?: string[],
  usage?: string
}

export class BaseCommand extends Command.Command {
  metadata!: CommandMetadata;

  responseOptional = true;

  constructor (commandClient: CommandClient, options: Partial<Command.CommandOptions>) {
    super(commandClient, Object.assign({
      name: '',
      ratelimits: [
        { duration: 5000, limit: 5, type: 'guild' },
        { duration: 1000, limit: 1, type: 'channel' }
      ]
    }, options));
  }

  get assyst () {
    return this.commandClient as Assyst;
  }

  async error (context: Command.Context, content: string) {
    return context.editOrReply({
      embed: {
        color: EmbedColors.ERROR,
        title: '⚠️ Command Error',
        description: content.slice(0, 1500)
      }
    });
  }

  public async getRecentAttachmentOrEmbed (msg: Message, amtOfMessages: number): Promise<string | undefined> {
    if (msg.attachments.length > 0) {
      return msg.attachments.first()?.url;
    }
    const messages: Array<Message> = await this.commandClient.rest.fetchMessages(msg.channelId, { limit: amtOfMessages });
    if (!messages) {
      return undefined;
    }
    let attachment: string | undefined;
    for (const message of messages) {
      if (message.attachments.length > 0) {
        // types broke
        // @ts-ignore
        return message.attachments[0].url;
      } else if (message.embeds.length > 0) {
        // types broke
        // @ts-ignore
        const embed: MessageEmbed | undefined = message.embeds[0];
        if (embed?.thumbnail) {
          return embed.thumbnail.url;
        } else if (embed?.image) {
          return embed.image.url;
        } else {
          continue;
        }
      }
    }
    return attachment;
  }

  public async getUrlFromChannel (ctx: Command.Context, args?: string): Promise<string | undefined> {
    let imageUrl: string | undefined;
    if (args) {
      imageUrl = args;
      try {
        const parsedURL: URL = new URL(<string>imageUrl);
        imageUrl = parsedURL.origin + parsedURL.pathname + parsedURL.search;
      } catch (e) {
        return undefined;
      }
    } else {
      imageUrl = await this.getRecentAttachmentOrEmbed(ctx.message, 50);
    }
    return imageUrl;
  }

  async uploadFile (data: any, contentType: string) {
    return uploadToTsu(data, contentType);
  }

  async userOwnsGuild (context: Context) {
    const guild = await context.rest.fetchGuild(context.guildId as string);

    return guild.ownerId === context.userId ||
          context.client.owners.map(u => u.id).includes(context.userId) ||
          admins.includes(context.userId);
  }

  parseMentionOrId (input: string) {
    const match = input.match(/^<@!?(\d{17,19})>/);
    if (match) {
      return match[1];
    } else {
      return input;
    }
  }

  async onBefore (context: Command.Context): Promise<boolean> {
    const oldEditOrReply: ((options: Command.EditOrReply | string) => Promise<Message>) = context.editOrReply.bind(context);

    context.editOrReply = (options?: string | Command.EditOrReply) => {
      if (typeof options === 'string') {
        return oldEditOrReply({
          content: options,
          allowedMentions: {
            parse: []
          }
        });
      } else {
        return oldEditOrReply({
          ...options,
          allowedMentions: {
            parse: []
          }
        });
      }
    };

    return true;
  }

  async onRunError (context: Command.Context, _: any, error: any) {
    const commandClient = context.commandClient as Assyst;

    const description: string[] = [error.message || error.stack];

    if (error.response) {
      const response: Response = error.response;
      try {
        const information = await response.json() as any;
        if ('errors' in information) {
          for (const key in information.errors) {
            const value = information.errors[key];
            let message: string;
            if (typeof (value) === 'object') {
              message = JSON.stringify(value);
            } else {
              message = String(value);
            }
            description.push(`**${key}**: ${message}`);
          }
        }
      } catch (e) {
        description.push(await response.text());
      }
    }

    await commandClient.executeLogWebhook(logWebhooks.commandErrors, {
      embed: {
        color: EmbedColors.ERROR,
        description: description.join('\n').slice(0, 1500),
        fields: [
          {
            name: 'Command',
            value: context.command?.name || '',
            inline: true
          }
        ],
        title: '⚠️ Command Error'
      }
    });

    await this.error(context, description.join('\n').slice(0, 1500));
  }

  onTypeError (context: Command.Context, args: any, errors: Command.ParsedErrors) {
    const store: { [key: string]: string } = {};

    const description: Array<string> = ['Invalid Arguments' + '\n'];
    for (const key in errors) {
      const message = errors[key].message;
      if (message in store) {
        description.push(`**${key}**: Same error as **${store[message]}**`);
      } else {
        description.push(`**${key}**: ${message}`);
      }
      store[message] = key;
    }

    return context.editOrReply({
      embed: {
        color: EmbedColors.ERROR,
        description: description.join('\n').slice(0, 1500),
        title: '⚠️ Command Argument Error'
      }
    });
  }
}
