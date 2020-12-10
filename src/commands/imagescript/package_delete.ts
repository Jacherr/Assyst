import { BaseFapiCommand } from '../basefapicommand';
import { Context, EditOrReply } from 'detritus-client/lib/command';

export interface CommandArgs {
    name: string;
}

export default class ImageScriptPackageDeleteCommand extends BaseFapiCommand {
    aliases = ['ist package delete']

    label = 'name'

    name = 'imagescripttag package delete'

    metadata = {
      description: 'Delete an ImageScript package',
      examples: ['test' + Math.random().toFixed(3)],
      usage: '[package name]'
    }

    priority = 2

    async run (context: Context, args: CommandArgs) {
      if (!args.name) {
        return this.error(context, 'No package name was specified.');
      }

      const tag = await this.assyst.database.fetchImageScriptPackage(args.name);

      if (!tag || tag.owner !== context.userId) {
        return this.error(context, 'This package either doesn\'t exist or you don\'t own it.');
      }

      this.assyst.database.deleteImageScriptPackage(tag.name);

      context.editOrReply('Package deleted successfully.');
    }
}
