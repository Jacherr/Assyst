import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import { SearchResultEntry } from '../../rest/zx8/types';

export interface CommandArgs {
  bomb: boolean,
  type: string
}

export default class Zx8RandomCommand extends BaseCommand {
  args = [
    {
      name: 'bomb',
      type: Boolean,
      default: false
    }
  ]

  name = 'zx8 random'

  label = 'type'

  metadata = {
    description: 'Get random zx8 thing',
    examples: ['image'],
    usage: '[image|html|video|audio]'
  }

  async run (context: Context, args: CommandArgs) {
    await context.triggerTyping();

    let data: SearchResultEntry[];
    let ext: string;

    switch (args.type.toLowerCase()) {
      case 'audio': {
        data = await this.assyst.zx8.randomAudio();
        ext = 'mp3';
        break;
      }
      case 'video': {
        data = await this.assyst.zx8.randomVideo();
        ext = 'mp4';
        break;
      }
      case 'image': {
        data = await this.assyst.zx8.randomImage();
        ext = 'png';
        break;
      }
      case 'html': {
        data = await this.assyst.zx8.randomHtml();
        const site = data[0];
        let screenshot: Buffer;

        try {
          screenshot = await this.assyst.screenshot(site.url, context.channel?.nsfw, 0);
        } catch (e) {
          return this.error(context, `${site.url}\n\n${e.message}`);
        }

        return context.editOrReply({
          content: `<${site.url}>`,
          file: {
            filename: 'zx8.png',
            value: screenshot
          }
        });
      }
      default: {
        return this.error(context, 'Specify image, video, html or audio.');
      }
    }

    const urls: string[] = [];
    const limit = (args.bomb ? 5 : 1);

    const promises: Promise<any>[] = [];

    for (let i = 0; i < limit; i++) {
      if (data[i]) {
        urls.push(`<${data[i].url}>`);
        promises.push(context.rest.request({ url: data[i].url, timeout: 5000 }));
      }
    }

    let files;
    try {
      files = (await Promise.all(promises))
        .map((x, i) => ({
          filename: `zx8_${i}.${ext}`,
          value: x
        }));
    } catch (e) {
      return this.error(context, `${urls.join('\n')}\n\n${e.message}`);
    }

    try {
      await context.reply({
        content: urls.join('\n'),
        files
      });
    } catch (e) {
      return this.error(context, `${urls.join('\n')}\n\n${e.message}`);
    }
  }
}
