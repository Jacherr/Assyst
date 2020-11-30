import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';

import { ContentTypes } from '../../rest/zx8/types';
import { EmbedColors } from '../../constants';
import { ReactionPaginator } from 'detritus-pagination';
import { Message } from 'detritus-client/lib/structures';

export interface CommandArgs {
    query: string;
    limit: number;
    ocr: boolean;
    offset: number
}

export default class Zx8ImageCommand extends BaseCommand {
    aliases = ['zx8 img', 'zx8 i']

    name = 'zx8 image'

    label = 'query'

    args = [
      {
        name: 'limit',
        type: Number,
        default: 10
      },
      {
        name: 'ocr',
        type: Boolean,
        default: false
      },
      {
        name: 'offset',
        type: Number,
        default: 0
      }
    ]

    metadata = {
      description: 'Search the zx8 web scraper',
      examples: ['meme'],
      usage: '[query]'
    }

    async run (context: Context, args: CommandArgs) {
      if (args.limit > 100) {
        return this.error(context, 'Limit must 100 or less');
      }

      const res = await this.assyst.zx8.search({
        query: args.query,
        limit: args.limit,
        offset: args.offset,
        ct: ContentTypes.IMAGE,
        ocr: args.ocr
      });

      if (res.length === 0) {
        return this.error(context, 'No results found');
      }

      const items = res.map(r => r.url);

      const pages = items.map(i => {
        return {
          embed: {
            image: {
              url: i
            },
            color: EmbedColors.ZX8
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
