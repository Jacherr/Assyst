import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    url: string
}

export default class IFunnyCommand extends BaseFapiCommand {
    label = 'url'

    name = 'ifunny'

    metadata = {
      description: 'I Funny',
      examples: ['https://link.to.my/image.png'],
      usage: '[url|attachment]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const url = await this.getUrlFromChannel(context, args.url);
      if (!url) {
        return this.error(context, 'No valid URL was found... Please use an attachment or valid image URL');
      }
      const res = await this.fapi.iFunny(url);
      return context.editOrReply({
        file: {
          filename: 'ifunny.png',
          value: res
        }
      });
    }
}
