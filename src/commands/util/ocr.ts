import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import fetch from 'node-fetch';
import { Markup } from 'detritus-client/lib/utils';
import { ocrImage } from '../../rest/rest';

interface CommandArgs {
    link: string
}

export default class CreplsCommand extends BaseCommand {
    name = 'ocr'

    label = 'link'

    metadata = {
      description: 'do optical character recognition on an image',
      examples: ['https://link.to.my/image.png'],
      usage: '<attachment|link>'
    }

    async run (context: Context, args: CommandArgs) {
        const url = await this.getUrlFromChannel(context, args.link)
        if(!url) return this.error(context, 'No valid URL was found... Please use an attachment or valid image URL')
        await context.triggerTyping();
        const result = await ocrImage(url);
        if(!result) return this.error(context, 'No text detected')
        return context.editOrReply(Markup.codeblock(result));
    }
}
