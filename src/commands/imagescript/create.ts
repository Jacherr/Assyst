import { BaseImageScriptCommand } from '../baseimagescriptcommand';
import { Context, EditOrReply } from 'detritus-client/lib/command';

export interface CommandArgs {
    args: string;
}

export default class ImageScriptCreateCommand extends BaseImageScriptCommand {
    aliases = ['ist create']

    label = 'args'

    name = 'imagescripttag create'

    metadata = {
      description: 'Create a new ImageScript tag',
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

      if (foundTag) {
        return this.error(context, 'This tag already exists.');
      }

      const code = await this.loadCode(context, content);

      if (!code) {
        return this.error(context, 'No tag content was specified.');
      } else if (code.length > 10000) {
        return this.error(context, 'ImageScript tags cannot be longer than 10,000 characters.');
      }

      await this.assyst.database.createImageScriptTag(tag, code, context.userId);
      context.editOrReply('Tag created successfully.');
    }
}
