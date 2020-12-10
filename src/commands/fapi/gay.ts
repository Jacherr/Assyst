import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    url: string
}

export default class GayCommand extends BaseFapiCommand {
    label = 'url'

    name = 'gay'

    metadata = {
      description: 'Gay',
      examples: ['https://link.to.my/image.png'],
      usage: '[url|attachment]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const url = await this.getUrlFromChannel(context, args.url);
      if (!url) {
        return this.error(context, 'No valid URL was found... Please use an attachment or valid image URL');
      }
      const res = await this.fapi.gay(url);
      return context.editOrReply({
        file: {
          filename: 'gay.png',
          value: res
        }
      });
    }
}
