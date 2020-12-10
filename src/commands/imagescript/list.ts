import { BaseFapiCommand } from '../basefapicommand';
import { Context } from 'detritus-client/lib/command';
import { generateTable, splitArray } from '../../utils';
import { Markup } from 'detritus-client/lib/utils';
import { ReactionPaginator } from 'detritus-pagination';
import { Message } from 'detritus-client/lib/structures';

export default class ImageScriptListCommand extends BaseFapiCommand {
    aliases = ['ist list']

    name = 'imagescripttag list'

    metadata = {
      description: 'Fetch a list of all ImageScript tags you own',
      examples: ['me'],
      usage: '[tag name]'
    }

    priority = 2

    async run (context: Context) {
      const userTags = await this.assyst.database.fetchUserImageScriptTags(context.userId);

      if (userTags.length === 0) {
        return this.error(context, 'You don\'t own any tags.');
      }

      const lists = splitArray(userTags.map(t => [t.name, t.uses]), 10);
      const pages = [];

      for (const page of lists) {
        pages.push(Markup.codeblock(generateTable({
          offset: 4,
          header: ['Name', 'Uses'],
          rows: page
        }), {
          language: 'hs'
        }));
      }
      
      const paginator = await this.assyst.paginator.createReactionPaginator({
        pages,
        message: context
      });

      this.assyst.replies.set(context.messageId, {
        command: this,
        context,
        reply: paginator.commandMessage as Message
      });
    }
}
