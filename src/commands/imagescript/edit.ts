import { BaseFapiCommand } from '../basefapicommand';
import { Context, EditOrReply } from 'detritus-client/lib/command';
import { parseCodeblocks } from '../../utils';
import { Attachment } from 'detritus-client/lib/structures';

export interface CommandArgs {
    args: string;
}

export default class ImageScriptEditCommand extends BaseFapiCommand {
    aliases = ['ist edit']

    label = 'args'

    name = 'imagescripttag edit'

    metadata = {
      description: 'Edit an existing ImageScript tag',
      examples: ['test' + Math.random().toFixed(3) + ' const image = Image.new(1000, 1000)'],
      usage: '[tag name] [tag content]'
    }

    priority = 2

    async run (context: Context, args: CommandArgs) {
      const [tag, content] = this.parseImageScriptArgs(args.args);

      if (!tag) {
        return this.error(context, 'No tag name was specified.');
      }

      const foundTag = await this.assyst.database.fetchImageScriptTag(tag);

      if (!foundTag || foundTag.owner !== context.userId) {
        return this.error(context, 'This tag either doesn\'t exist or you don\'t own it.');
      }

      const code = await this.loadCode(context, content);

      if (!code) {
        return this.error(context, 'No tag content was specified.');
      } else if (code.length > 10000) {
        return this.error(context, 'ImageScript tags cannot be longer than 10,000 characters.');
      }

      await this.assyst.database.editImageScriptTag(tag, code, context.userId, foundTag.uses);
      context.editOrReply('Tag edited successfully.');
    }
}
