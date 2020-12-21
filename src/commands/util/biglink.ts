import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import { Message } from 'detritus-client/lib/structures';

interface CommandArgs {
    link: string
}

function getProxyUrlFromEmbed(message: Message): string {
  // We assert it because we know it will have a proxy url
  return message.embeds.first()!.image!.proxyUrl!;
}

async function generateBigLink(context: Context, url: string): Promise<string> {
  const initial = await context.reply({ embed: { image: { url } } });
  
  let previous: string = getProxyUrlFromEmbed(initial), counter = 0;
  
  // We use a counter variable to ensure it never edits more than 10 times
  // This is to prevent infinite loops (in case it somehow never exceeds 2000 characters)
  while (counter++ < 25) {
    const next = await initial.edit({ embed: { image: { url: previous } } });
    
    const url = getProxyUrlFromEmbed(next);
    if (url.length > 2000) break;
    previous = url;
  }

  await initial.delete();

  return previous;
}

export default class BiglinkCommand extends BaseCommand {
    name = 'biglink'

    label = 'link'

    metadata = {
      description: 'generates a big link',
      examples: ['https://link.to.my/image.png'],
      usage: '<attachment|link>'
    }

    async run (context: Context, args: CommandArgs) {
        const url = await this.getUrlFromChannel(context, args.link);
        if (!url) return context.editOrReply('URL not found');
        const link = await generateBigLink(context, url);
        return context.editOrReply(`<${link}>`);
    }
}
