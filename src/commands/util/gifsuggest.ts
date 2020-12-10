import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import { fetchGifSuggestions } from '../../rest/rest';
import { Markup } from 'detritus-client/lib/utils';

export interface CommandArgs {
    query: string
}

export default class GifCommand extends BaseCommand {
    aliases = ['suggest']

    label = 'query'

    name = 'gifsuggest'

    metadata = {
      description: 'Fetch GIF suggestions'
    }

    async run (context: Context, args: CommandArgs) {
      if (!args.query) {
        return this.error(context, 'Provide a search query.');
      }
      const results = await fetchGifSuggestions(args.query);
      if(results.length === 0) {
          return this.error(context, 'No results found.')
      }
      return context.reply(Markup.codeblock(results.join('\n')));
    }
}