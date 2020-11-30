import { BaseFapiCommand } from '../basefapicommand';
import { Context, EditOrReply } from 'detritus-client/lib/command';
import { Markup } from 'detritus-client/lib/utils';

export interface CommandArgs {
    name: string;
    attach: boolean;
}

export default class ImageScriptPackageRawCommand extends BaseFapiCommand {
    aliases = ['ist package raw']

    args = [
      {
        name: 'upload',
        default: false,
        type: Boolean
      }
    ]

    label = 'name'

    name = 'imagescripttag package raw'

    metadata = {
      description: 'Fetch the raw content of an ImageScript package',
      examples: ['random'],
      usage: '[tag name]'
    }

    priority = 2

    async run (context: Context, args: CommandArgs) {
      if (!args.name) {
        return this.error(context, 'No package name was specified.');
      }

      const code = await this.assyst.database.fetchImageScriptPackage(args.name);

      if (!code) {
        return this.error(context, 'No package with this name was found.');
      }

      let output: EditOrReply;

      if (code.content.length > 1995 || args.attach) {
        output = {
          content: await this.uploadFile(code.content, 'application/javascript')
        };
      } else {
        output = {
          content: Markup.codeblock(code.content, {
            language: 'js'
          })
        };
      }

      return context.editOrReply(output);
    }
}
