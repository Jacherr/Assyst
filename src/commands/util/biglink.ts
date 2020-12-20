import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import { Message } from 'detritus-client/lib/structures';

interface CommandArgs {
    link: string
}

function getProxyUrlFromEmbed(message: Message) {
  return message.embeds.first().image.proxyUrl;
}

async function generateBigLink(context: Context, url: string): Promise<string> {
  const initial = await context.reply({ embed: { image: { url } } });
  
  let previous: string = getProxyUrlFromEmbed(initial);
  
  while (true) {
    const next = await initial.edit({ embed: { image: { url: previous } } });
    
    const url = getProxyUrlFromEmbed(next);
    if (url.length > 2000) break;
    previous = url;
  }
  
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
        const link = await generateBigLink(context, args.link);
        return context.editOrReply(link);
    }
}
