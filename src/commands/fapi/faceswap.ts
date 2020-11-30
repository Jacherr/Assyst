import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    url: string
}

export default class FaceSwapCommand extends BaseFapiCommand {
    label = 'url'

    name = 'faceswap'

    metadata = {
      description: 'Face Swap',
      examples: ['https://link.to.my/image.png'],
      usage: '[url|attachment]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const url = await this.getUrlFromChannel(context, args.url);
      if (!url) {
        return this.error(context, 'No valid URL was found... Please use an attachment or valid image URL');
      }
      const res = await this.fapi.faceSwap(url);
      return context.editOrReply({
        file: {
          filename: 'faceswap.png',
          value: res
        }
      });
    }
}
