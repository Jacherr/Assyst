import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';

import { generateKVList } from '../../utils';
import { Markup } from 'detritus-client/lib/utils';
import { EmbedColors } from '../../constants';

export default class Zx8InfoCommand extends BaseCommand {
    name = 'zx8 info'

    metadata = {
      description: 'Get information about the zx8 web scraper'
    }

    async run (context: Context) {
      const start = Date.now();
      const res = await this.assyst.zx8.info();
      const time = Date.now() - start;

      const table = generateKVList([
        ['URL Queue', res.urlQueue.toLocaleString()],
        ['Total URLs', res.totalURLs.toLocaleString()],
        ['RSS', `${(res.rss/1024/1204).toFixed(2)}MiB`],
        ['Table Size', `${(res.tableSize/1000).toLocaleString()}GB`],
        ['Indexes Per Second', res.indexesPerSecond.toLocaleString()],
        ['', ''],
        ['Images', res.contentTypes.image.toLocaleString()],
        ['GIFs', res.contentTypes.animated.toLocaleString()],
        ['Videos', res.contentTypes.video.toLocaleString()],
        ['HTML Documents', res.contentTypes.html.toLocaleString()],
        ['Other Documents', res.contentTypes.other.toLocaleString()]
      ]);

      return context.editOrReply({
        embed: {
          color: EmbedColors.ZX8,
          description: Markup.codeblock(table, {
            language: 'ml'
          }),
          footer: {
            text: `Took ${time}ms`
          },
          title: 'zx8 information'
        }
      });
    }
}
