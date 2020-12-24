import { BaseFapiCommand } from '../basefapicommand';
import { Context, EditOrReply } from 'detritus-client/lib/command';
import { Markup } from 'detritus-client/lib/utils'

import { executeImageScript, IsapiData } from '../../rest/rest';

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

      let response: IsapiData;

      try {
        response = await executeImageScript(code.content, {
          avatar: context.user.avatarUrl,
          args: tagArgs
        });
      } catch (e) {
        return context.editOrReply(e.message);
      }
  
      const guildAttachmentLimitBytes = await context.rest.fetchGuild(<string>context.guildId).then(g => g.maxAttachmentSize);
  
      let output: EditOrReply = {};
      output.content = "";
  
      if (args.m) {
        output.content = [
          `**CPU Time**: \`${response.cpuTime.toFixed(1)}ms\``,
          `**Wall Time**: \`${response.wallTime.toFixed(1)}ms\``,
        ].join('\n');
        if (response.image) {
          output.content += `\n**Image Size**: \`${(response.image.length / 1000 / 1000).toFixed(1)} MB\``;
        }
      }

      if (response.text) {
        output.content += `\n**Text**: ${Markup.codeblock(response.text)}`;
      }
  
      if (response.image) {
        if (response.image.length > guildAttachmentLimitBytes || response.format === 'gif') {
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
      }

      await context.editOrReply(output);
      this.assyst.database.setImageScriptTagUses(code.name, code.uses + 1)
    }
}
