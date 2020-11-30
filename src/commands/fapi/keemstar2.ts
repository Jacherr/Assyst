import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    url: string
}

export default class Keemstar2Command extends BaseFapiCommand {
    label = 'url'

    name = 'keemstar2'

    metadata = {
      description: 'Keemstar 2',
      examples: ['https://link.to.my/image.png'],
      usage: '[url|attachment]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const url = await this.getUrlFromChannel(context, args.url);
      if (!url) {
        return this.error(context, 'No valid URL was found... Please use an attachment or valid image URL');
      }
      const res = await this.fapi.keemstar2(url);
      return context.editOrReply({
        file: {
          filename: 'keemstar2.png',
          value: res
        }
      });
    }
}
