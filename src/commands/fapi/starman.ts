import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    url: string
}

export default class StarmanCommand extends BaseFapiCommand {
    label = 'url'

    name = 'starman'

    metadata = {
      description: 'Starman',
      examples: ['https://link.to.my/image.png'],
      usage: '[url|attachment]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const url = await this.getUrlFromChannel(context, args.url);
      if (!url) {
        return this.error(context, 'No valid URL was found... Please use an attachment or valid image URL');
      }
      const res = await this.fapi.starman(url);
      return context.editOrReply({
        file: {
          filename: 'starman.png',
          value: res
        }
      });
    }
}
