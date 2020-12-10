import { BaseFapiCommand } from '../basefapicommand';
import { Context } from 'detritus-client/lib/command';
import { generateKVList } from '../../utils';
import { Markup } from 'detritus-client/lib/utils';

export interface CommandArgs {
    name: string;
}

export default class ImageScriptInfoCommand extends BaseFapiCommand {
    aliases = ['ist info']

    label = 'name'

    name = 'imagescripttag info'

    metadata = {
      description: 'Fetch the info of an ImageScript tag',
      examples: ['me'],
      usage: '[tag name]'
    }

    priority = 2

    async run (context: Context, args: CommandArgs) {
      if (!args.name) {
        return this.error(context, 'No tag name was specified.');
      }

      const tag = await this.assyst.database.fetchImageScriptTag(args.name);

      if (!tag) {
        return this.error(context, 'No tag with this name was found.');
      }

      const output = generateKVList([
        ['Name', tag.name],
        ['Owner', await context.rest.fetchUser(tag.owner).then(u => `${u.username}#${u.discriminator}`)],
        ['Uses', tag.uses.toString()]
      ]);

      return context.editOrReply(Markup.codeblock(output, {
        language: 'ml'
      }));
    }
}
