import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';

import { generateTable } from '../../utils';
import { Markup } from 'detritus-client/lib/utils';

export default class Zx8NodesCommand extends BaseCommand {
    name = 'zx8 nodes'

    metadata = {
      description: 'Get information about the zx8 web scraper\'s nodes'
    }

    async run (context: Context) {
      const res = await this.assyst.zx8.nodes();

      const rows = res.map((r, index) => {
        return [index, `${r.ping}ms`, `${(r.memory/1024/1024).toFixed(2)}MiB`, r.available ? 'Yes' : 'No', r.queue];
      });

      const table = generateTable({
        offset: 4,
        header: ['#', 'Ping', 'RSS', 'Available', 'URL Queue'],
        rows
      });

      return context.editOrReply(Markup.codeblock(table, {
        language: 'md'
      }));
    }
}
