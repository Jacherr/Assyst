import { BaseImageScriptCommand } from '../baseimagescriptcommand';
import { Context, EditOrReply } from 'detritus-client/lib/command';
import { executeImageScript, IsapiData } from '../../rest/rest';
import { Markup } from 'detritus-client/lib/utils';

export interface CommandArgs {
  code: string;
  m: boolean
}

export default class ImageScriptCommand extends BaseImageScriptCommand {
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

  async run(context: Context, args: CommandArgs) {
    let code = await this.loadCode(context, args.code);

    code = await this.injectImageScriptPackages(code);

    let response: IsapiData;

    try {
      response = await executeImageScript(code, {
        avatar: context.user.avatarUrl
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
      output.content += `${Markup.codeblock(response.text.slice(0, 1900))}`;
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

    return context.editOrReply(output);
  }
}
