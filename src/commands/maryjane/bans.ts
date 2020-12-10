import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import { generateKVList, generateTable } from '../../utils';

import { Markup } from 'detritus-client/lib/utils'

export interface CommandArgs {
  guildId: string
}

export default class MaryjaneBansCommand extends BaseCommand {
    aliases = ['mj bans']

    name = 'maryjane bans'

    label = 'guildId'

    metadata = {
      description: 'Get bans for a guild from Maryjane API',
      examples: ['178313653177548800'],
      usage: '[guild id]'
    }

    async run (context: Context, args: CommandArgs) {
      const guild = await this.assyst.maryjane.guild(args.guildId || context.guildId as string);
      const bans = guild.bans;
      let output;

      if(bans.length === 0) {
          return this.error(context, 'This guild has no bans.')
      } else {
          let table = generateTable({
              offset: 4,
              header: ['#', 'User ID', 'Reason'],
              rows: bans.map((b, i) => [i, b.userid, b.reason ?? 'N/A']).slice(0, 15)
          })

          output = Markup.codeblock(table, { language: 'md' })
      }

      return context.editOrReply(output);
    }
}
