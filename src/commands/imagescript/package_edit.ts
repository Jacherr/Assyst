import { BaseFapiCommand } from '../basefapicommand';
import { Context, EditOrReply } from 'detritus-client/lib/command';
import { parseCodeblocks } from '../../utils';
import { packageWhitelist } from '../../../config.json';

export interface CommandArgs {
    args: string;
}

export default class ImageScriptPackageEditCommand extends BaseFapiCommand {
    aliases = ['ist package edit']

    label = 'args'

    name = 'imagescripttag package edit'

    metadata = {
      description: 'Edit an existing ImageScript package',
      examples: ['test' + Math.random().toFixed(3) + ' globalThis.a = 1'],
      usage: '[package name] [package content]'
    }

    priority = 3

    async onBefore (context: Context) {
      if (!packageWhitelist.includes(context.userId) && !context.user.isClientOwner) return false;
      return super.onBefore(context);
    }

    async run (context: Context, args: CommandArgs) {
      const [packageName, content] = this.parseImageScriptArgs(args.args);

      if (!packageName) {
        return this.error(context, 'No package name was specified.');
      }

      const foundTag = await this.assyst.database.fetchImageScriptPackage(packageName);

      if (!foundTag || foundTag.owner !== context.userId) {
        return this.error(context, 'This package either doesn\'t exist or you don\'t own it.');
      }

      const code = await this.loadCode(context, content);

      if (!code) {
        return this.error(context, 'No package content was specified.');
      }

      await this.assyst.database.editImageScriptPackage(packageName, code.trim(), context.userId);
      context.editOrReply('Package edited successfully.');
    }
}
