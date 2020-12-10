import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';

import { generateTable, flat } from '../../utils';
import { Markup } from 'detritus-client/lib/utils';

export interface CommandArgs {
    query: string;
    limit: number;
    ct: string;
    offset: number
}

export default class Zx8SearchCommand extends BaseCommand {
    name = 'zx8 search'

    label = 'query'

    args = [
      {
        name: 'limit',
        type: Number,
        default: 10
      },
      {
        name: 'ct',
        type: String,
        default: 'all'
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
      const ctAssociations: { [key: string]: any } = {
        all: '',
        other: 0,
        image: 1,
        animated: 2,
        gif: 2,
        video: 3,
        html: 4
      };

      if (args.limit > 50) {
        return this.error(context, 'Limit not exceed 50 items.');
      }

      const res = await this.assyst.zx8.search({
        query: args.query,
        limit: args.limit,
        offset: args.offset,
        ct: ctAssociations[args.ct] || undefined,
        ocr: false
      });

      if (res.length === 0) {
        return this.error(context, 'No results found');
      }

      const rows = res.map((r, i) => {
        return [i, r.url.trim().slice(0, 125), r.lastStatus];
      });

      const rawPages = flat(rows, 5);

      const formattedPages = rawPages.map(p => generateTable({
        offset: 4,
        header: ['#', 'URL', 'Status'],
        rows: p
      }))

      const codeblockedPages = formattedPages.map(p => {
        return Markup.codeblock(p, {
            language: 'md'
        })
      })

      return await this.assyst.paginator.createReactionPaginator({
        pages: codeblockedPages,
        message: context
      });
    }
}
