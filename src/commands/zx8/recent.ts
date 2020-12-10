import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';

import { generateTable, flat } from '../../utils';
import { Markup } from 'detritus-client/lib/utils';

export default class Zx8RecentCommand extends BaseCommand {
    name = 'zx8 recent'

    metadata = {
      description: 'Get recently indexed urls from zx8'
    }

    async run (context: Context) {
      const res = await this.assyst.zx8.recentIndexes();

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
