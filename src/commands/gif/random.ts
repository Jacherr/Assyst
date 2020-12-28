import { BaseAdminCommand } from '../baseadmincommand';
import { Context } from 'detritus-client/lib/command';

import { inflate } from 'zlib';
import { uploadToTsu } from '../../rest/rest';


export default class GifAddCommand extends BaseAdminCommand {
    aliases = ['gc random', 'gifcatalogue random']

    name = 'gifc random'

    priority = 2

    metadata = {
      description: 'get random gif',
    }

    async run (context: Context) {
        const gifs = await this.assyst.database.fetchAllGifs();
        const gif = gifs[Math.floor(Math.random()*gifs.length)];

        const decompressedBuf: Buffer = await new Promise((resolve, reject) => inflate(gif.buffer, (err: Error | null, buf: Buffer) => {
            if(err) reject(err);
            resolve(buf);
        }))

        const guildAttachmentLimitBytes = await context.rest.fetchGuild(<string>context.guildId).then(g => g.maxAttachmentSize);

        if(guildAttachmentLimitBytes > decompressedBuf.length) {
            return context.editOrReply({
                content: `Keywords: \`${gif.keywords}\``,
                file: {
                    filename: 'gif.gif',
                    value: decompressedBuf
                }
            })
        } else {
            const url = await uploadToTsu(decompressedBuf, 'image/gif');
            return context.editOrReply(`Keywords: \`${gif.keywords}\`\n${url}`);
        }
    }
}
