import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import { fetchGifs } from '../../rest/rest';

export interface CommandArgs {
    query: string
}

export default class GifCommand extends BaseCommand {
    label = 'query'

    name = 'gif'

    metadata = {
      description: 'Fetch GIFs'
    }

    async run (context: Context, args: CommandArgs) {
      if (!args.query) {
        return this.error(context, 'Provide a search query.');
      }
      const results = await fetchGifs(args.query);
      if(results.length === 0) {
          return this.error(context, 'No results found.')
      }
      return context.reply(results[Math.floor(Math.random() * results.length)]);
    }
}