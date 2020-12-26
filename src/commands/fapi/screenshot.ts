import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

import { createHash } from 'crypto';

export interface CommandArgs {
    url: string,
    wait: number
}

export default class ScreenshotCommand extends BaseFapiCommand {
    label = 'url'

    name = 'screenshot'

    aliases = ['ss']

    args = [
      {
        name: 'wait',
        default: '0',
        type: Number
      }
    ]

    metadata = {
      description: 'Screenshot a webpage',
      examples: ['https://jacher.io/'],
      usage: '[url]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const c = await context.rest.fetchChannel(context.channelId);
      const url = await this.getUrlFromChannel(context, args.url);

      const res = await this.assyst.screenshot(url ?? args.url, c.nsfw, args.wait);

      return context.editOrReply({
        file: {
          filename: 'screenshot.png',
          value: res
        }
      });
    }
}
