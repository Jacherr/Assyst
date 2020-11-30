import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import { ContentTypes } from '../../rest/zx8/types';
import { url } from 'inspector';

export interface CommandArgs {
    query: string
}

export default class Zx8InfoCommand extends BaseCommand {
    aliases = ['zx8 page']

    name = 'zx8 html'

    label = 'query'

    metadata = {
      description: 'Search for and screenshot a single html web page',
      examples: ['reddit'],
      usage: '[query]'
    }

    async run (context: Context, args: CommandArgs) {
      await context.triggerTyping();

      const { query } = args;
      const result = await this.assyst.zx8.search({
        ct: ContentTypes.HTML,
        query
      });

      if (result.length === 0) {
        return this.error(context, 'No results found');
      }

      let screenshot;

      try {
        screenshot = await this.assyst.screenshot(result[0].url, context.channel?.nsfw ?? false, 0);
      } catch (e) {
        return this.error(context, `${result[0].url}\n\n${e.message}`);
      }

      return context.editOrReply({
        content: `<${result[0].url}>`,
        file: {
          filename: 'screenshot.png',
          value: screenshot
        }
      });
    }
}
