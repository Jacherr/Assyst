import { BaseCommand } from '../basecommand';
import { Command } from 'detritus-client';

import { uploadFile } from '../../rest/rest';

import fetch from 'node-fetch';

const GIF = new Uint8Array([71, 73, 70]);
const JPEG = new Uint8Array([255, 216, 255]);

export interface CommandArgs {
    url: string
}

export default class AmericaCommand extends BaseCommand {
    label = 'url'

    name = 'caption'

    metadata = {
      description: 'Caption image',
      examples: ['https://link.to.my/image.png yeah'],
      usage: '[url|attachment] [text]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const url = await this.getUrlFromChannel(context, args.url.split(' ')[0]);
      if (!url) {
        return this.error(context, 'No valid URL was found... Please use an attachment or valid image URL');
      }
      await context.triggerTyping();
      const text = args.url.split(' ')[1] ?? 'when you dont provide any text';
      const out = await fetch('https://wsi.jacher.io/caption?text=' + encodeURIComponent(text), { method: 'POST', headers: { authorization: '0192837465' }, body: await context.rest.request(url) });
      if (out.status !== 200) {
        return this.error(context, await out.text());
      }
      let format;
      const buffer = await out.buffer();
      if (buffer.slice(0, 3).equals(GIF)) format = 'gif';
      else if (buffer.slice(0, 3).equals(JPEG)) format = 'jpeg';
      else format = 'png';
      const finalUrl = await uploadFile(buffer, `image/${format}`);
      context.editOrReply(finalUrl);
    }
}
