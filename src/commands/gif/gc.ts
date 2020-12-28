import { BaseAdminCommand } from '../baseadmincommand';
import { Context } from 'detritus-client/lib/command';

import { inflate } from 'zlib';
import { uploadToTsu } from '../../rest/rest';

export interface CommandArgs {
    keywords: string,
    page: string
}

export default class GifAddCommand extends BaseAdminCommand {
    aliases = ['gc', 'gifcatalogue']

    args = [
        {
            name: 'page',
            type: String
        }
    ]

    name = 'gifc'

    label = 'keywords'

    metadata = {
      description: 'Get a gif from keywords',
      examples: ['yeah lol funny haha'],
      usage: '[keywords]'
    }

    async run (context: Context, args: CommandArgs) {
        if(!args.keywords) return this.error(context, 'Provide some keywords.');
        const splitKeywords = args.keywords.split(' ');
        const gifs = await this.assyst.database.fetchGifsFromKeywords(splitKeywords);
        if(gifs.length === 0) return this.error(context, 'No gifs found.')
        const page = isNaN(parseInt(args.page)) ? 1 : parseInt(args.page);
        if(page > gifs.length) return this.error(context, `Cannot fetch page ${page} of ${gifs.length} gifs`)
        const decompressedBuf: Buffer = await new Promise((resolve, reject) => inflate(gifs[page - 1].buffer, (err: Error | null, buf: Buffer) => {
            if(err) reject(err);
            resolve(buf);
        }))

        const guildAttachmentLimitBytes = await context.rest.fetchGuild(<string>context.guildId).then(g => g.maxAttachmentSize);

        if(guildAttachmentLimitBytes > decompressedBuf.length) {
            return context.editOrReply({
                content: `Entries: ${gifs.length}`,
                file: {
                    filename: 'gif.gif',
                    value: decompressedBuf
                }
            })
        } else {
            const url = await uploadToTsu(decompressedBuf, 'image/gif');
            return context.editOrReply(`Entries: ${gifs.length}\n${url}`);
        }
    }
}
