import { BaseFapiCommand } from '../basefapicommand';
import { Context, EditOrReply } from 'detritus-client/lib/command';
import { Markup } from 'detritus-client/lib/utils';

export interface CommandArgs {
    name: string;
    attach: boolean;
    packages: boolean
}

export default class ImageScriptRawCommand extends BaseFapiCommand {
    aliases = ['ist raw']

    args = [
      {
        name: 'upload',
        default: false,
        type: Boolean
      },
      {
        name: 'packages',
        default: false,
        type: Boolean
      }
    ]

    label = 'name'

    name = 'imagescripttag raw'

    metadata = {
      description: 'Fetch the raw content of an ImageScript tag',
      examples: ['me'],
      usage: '[tag name]'
    }

    priority = 2

    async run (context: Context, args: CommandArgs) {
      if (!args.name) {
        return this.error(context, 'No tag name was specified.');
      }

      const code = await this.assyst.database.fetchImageScriptTag(args.name);

      if (!code) {
        return this.error(context, 'No tag with this name was found.');
      }

      if (args.packages) {
        code.content = await this.injectImageScriptPackages(code.content);
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
