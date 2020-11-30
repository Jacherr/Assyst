import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import { generateKVList } from '../../utils';
import { Markup } from 'detritus-client/lib/utils';

import { cpus } from 'os';

export default class InfoCommand extends BaseCommand {
  aliases = ['stats']

  label = 'host'

  name = 'info'

  metadata = {
    description: 'Get information about Assyst'
  }

  async run(context: Context) {
    const result = generateKVList([
      [
        'Guilds',
        (await context.rest.fetchMeGuilds()).size.toString()
      ],
      [
        'Clusters',
        context.manager?.clusterCount.toString() || '1'
      ],
      [
        'Memory Usage',
        await context.manager?.broadcastEval('process.memoryUsage().rss / 1000 /1000').then((results: number[]) => {
          return results.reduce((a, b) => a + b).toFixed(1) + 'MB';
        }) as string
      ],
      [
        'Commands',
        this.assyst.commands.length.toString()
      ],
      [
        'Authors',
        'Jacher#9891, y21#0909'
      ],
      [
        'Database Size',
        await this.assyst.database.getDatabaseSize()
      ],
      [
        'CPU Model',
        `${cpus().length}x ${cpus()[0].model}`
      ]
    ]);

    return context.editOrReply(Markup.codeblock(result, {
      language: 'js'
    }));
  }
}
