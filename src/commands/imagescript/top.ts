import { BaseFapiCommand } from '../basefapicommand';
import { Context } from 'detritus-client/lib/command';
import { generateTable } from '../../utils';
import { Markup } from 'detritus-client/lib/utils';

export default class ImageScriptTopCommand extends BaseFapiCommand {
    aliases = ['ist top']

    name = 'imagescripttag top'

    metadata = {
      description: 'Fetch the info of an ImageScript tag',
      examples: ['me'],
      usage: '[tag name]'
    }

    priority = 2

    async run (context: Context) {
      const topTags = await this.assyst.database.fetchTopImageScriptTags();

      const rows = topTags.map(t => [t.name, t.uses, t.owner]);

      const table = generateTable({
        offset: 4,
        header: ['Name', 'Uses', 'Owner'],
        rows
      });

      return context.editOrReply(Markup.codeblock(table, {
        language: 'hs'
      }));
    }
}
