import { BaseImageScriptCommand } from '../baseimagescriptcommand';
import { Context, EditOrReply } from 'detritus-client/lib/command';

export interface CommandArgs {
    name: string;
}

export default class ImageScriptDeleteCommand extends BaseImageScriptCommand {
    aliases = ['ist delete']

    label = 'name'

    name = 'imagescripttag delete'

    metadata = {
      description: 'Delete an ImageScript tag',
      examples: ['test' + Math.random().toFixed(3)],
      usage: '[tag name]'
    }

    priority = 2

    async run (context: Context, args: CommandArgs) {
      if (!args.name) {
        return this.error(context, 'No tag name was specified.');
      }

      const tag = await this.assyst.database.fetchImageScriptTag(args.name);

      if (!tag || tag.owner !== context.userId) {
        return this.error(context, 'This tag either doesn\'t exist or you don\'t own it.');
      }

      this.assyst.database.deleteImageScriptTag(tag.name);

      context.editOrReply('Tag deleted successfully.');
    }
}
