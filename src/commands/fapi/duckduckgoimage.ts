import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';
import { Message } from 'detritus-client/lib/structures';
import { ReactionPaginator } from 'detritus-pagination';
import { EmbedColors } from '../../constants';

export interface CommandArgs {
  query: string
}

export default class DuckDuckGoImageCommand extends BaseFapiCommand {
  label = 'query'

  name = 'duckduckgoimage'

  aliases = ['ddgi', 'img']

  metadata = {
    description: 'Search Duck Duck Go Images',
    examples: ['cat'],
    usage: '[search query]'
  }

  async run(context: Command.Context, args: CommandArgs) {
    let results = await this.fapi.duckDuckGoImages(args.query,
      {
        safe: await context.rest.fetchChannel(context.channelId).then(c => c.nsfw)
      });

    if (results.length === 0) {
      return this.error(context, 'No results found');
    }

    const pages = results.map(i => {
      return {
        embed: {
          image: {
            url: i
          },
          color: EmbedColors.INFO
        }
      };
    });

    const paginator = await this.assyst.paginator.createReactionPaginator({
      pages,
      message: context
    });

    this.assyst.replies.set(context.messageId, {
      command: this,
      context,
      reply: paginator.commandMessage as Message
    });
  }
}
