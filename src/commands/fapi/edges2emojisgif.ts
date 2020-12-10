import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    url: string
}

export default class Edges2EmojisGifCommand extends BaseFapiCommand {
    label = 'url'

    name = 'edges2emojisgif'

    metadata = {
      description: 'Edges 2 Emojis Gif',
      examples: ['https://link.to.my/image.png'],
      usage: '[url|attachment]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const url = await this.getUrlFromChannel(context, args.url);
      if (!url) {
        return this.error(context, 'No valid URL was found... Please use an attachment or valid image URL');
      }
      const res = await this.fapi.edges2EmojisGif(url);
      return context.editOrReply({
        file: {
          filename: 'edges2emojisgif.gif',
          value: res
        }
      });
    }
}
