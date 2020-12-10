import { BaseFapiCommand } from '../basefapicommand';
import { Context, EditOrReply } from 'detritus-client/lib/command';
import { ReturnTypes } from 'fapi-client/JS/src/types';

export interface CommandArgs {
    args: string;
    m: boolean;
    upload: boolean;
}

export default class ImageScriptTagCommand extends BaseFapiCommand {
    aliases = ['ist']

    args = [
      {
        name: 'm',
        type: Boolean,
        default: false
      },
      {
        name: 'upload',
        type: Boolean,
        default: false
      }
    ]

    label = 'args'

    name = 'imagescripttag'

    metadata = {
      description: 'Run an ImageScript tag',
      examples: ['me'],
      usage: '[tag name] <tag args>'
    }

    async run (context: Context, args: CommandArgs) {
      const [tag, tagArgs] = this.parseImageScriptArgs(args.args);

      if (!tag) {
        return this.error(context, 'No tag name was specified.');
      }

      const code = await this.assyst.database.fetchImageScriptTag(tag);

      if (!code) {
        return this.error(context, 'No tag with this name was found.');
      }

      code.content = await this.injectImageScriptPackages(code.content);

      let response: ReturnTypes.ImageScript;

      try {
        response = await this.fapi.imageScript(code.content, {
          avatar: context.user.avatarUrl + '?size=1024',
          args: tagArgs
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

      if (response.image.length > guildAttachmentLimitBytes || args.upload) {
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
