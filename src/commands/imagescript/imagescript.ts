import { BaseFapiCommand } from '../basefapicommand';
import { Context, EditOrReply } from 'detritus-client/lib/command';
import { parseCodeblocks } from '../../utils';
import { Attachment } from 'detritus-client/lib/structures';
import { ReturnTypes } from 'fapi-client/JS/src/types';

export interface CommandArgs {
    code: string;
    m: boolean
}

export default class ImageScriptCommand extends BaseFapiCommand {
    aliases = ['is']

    args = [
      {
        name: 'm',
        type: Boolean,
        default: false
      }
    ]

    label = 'code'

    name = 'imagescript'

    metadata = {
      description: 'Run ImageScript scripts',
      examples: ['const image = Image.new(1000, 1000, 0xffffff)'],
      usage: '[script]'
    }

    async run (context: Context, args: CommandArgs) {
      let code = await this.loadCode(context, args.code);

      code = await this.injectImageScriptPackages(code);

      let response: ReturnTypes.ImageScript;

      try {
        response = await this.fapi.imageScript(code, {
          avatar: context.user.avatarUrl + '?size=1024'
        });
      } catch (e) {
        return context.editOrReply(e.message);
      }

      const guildAttachmentLimitBytes = await context.rest.fetchGuild(<string> context.guildId).then(g => g.maxAttachmentSize);

      let output: EditOrReply = {};

      if (args.m) {
        output.content = [
            `**CPU Time**: \`${response.cpuTime.toFixed(1)}ms\``,
            `**Wall Time**: \`${response.wallTime.toFixed(1)}ms\``,
            `**Memory Usage**: \`${response.memoryUsage.toFixed(1)} MB\``,
            `**Image Size**: \`${(response.image.length / 1000 / 1000).toFixed(1)} MB\``
        ].join('\n');
      }

      if (response.image.length > guildAttachmentLimitBytes) {
        output.content += '\n' + await this.uploadFile(response.image, `image/${response.format}`);
      } else {
        output = {
          ...output,
          file: {
            filename: 'imagescript.' + response.format,
            value: response.image
          }
        };
      }

      return context.editOrReply(output);
    }
}
