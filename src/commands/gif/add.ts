import { BaseAdminCommand } from '../baseadmincommand';
import { Context } from 'detritus-client/lib/command';
import fetch from 'node-fetch';

import { deflate } from 'zlib';

export interface CommandArgs {
    args: string
}

export default class GifAddCommand extends BaseAdminCommand {
    aliases = ['gc add', 'gifcatalogue add']

    name = 'gifc add'

    label = 'args'

    metadata = {
      description: 'Add gif to dumb',
      examples: ['https://funny.gif/funny.gif yeah lol funny haha'],
      usage: '[url] [keywords]'
    }

    async run (context: Context, args: CommandArgs) {
        const [url, ...keywords] = args.args.split(' ');
        if(!url) return this.error(context, 'You must supply a url.')
        if(keywords.length === 0) return this.error(context, 'You must supply at least 1 keyword.');

        const res = await fetch(url).then(x => x.buffer());
        const head = res.slice(0, 3);
        if(!head.equals(Buffer.from('GIF'))) return this.error(context, 'Input must be a valid GIF.');

        const compressedBuf: Buffer = await new Promise((resolve, reject) => deflate(res, (err: Error | null, buf: Buffer) => {
            if(err) reject(err);
            resolve(buf);
        }))

        await this.assyst.database.addGifToCatalogue(compressedBuf, keywords.join(' '));
        return context.editOrReply('GIF added successfully.')
    }
}
