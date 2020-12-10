import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import { generateKVList } from '../../utils';

import { Markup } from 'detritus-client/lib/utils'

export interface CommandArgs {
  userId: string
}

export default class MaryjaneUserCommand extends BaseCommand {
    aliases = ['mj user']

    name = 'maryjane user'

    label = 'userId'

    metadata = {
      description: 'Get a user from Maryjane API',
      examples: ['571661221854707713'],
      usage: '[user id]'
    }

    async run (context: Context, args: CommandArgs) {
      const user = await this.assyst.maryjane.user(args.userId?.replace(/[<>@!]/g, '') || context.userId);

      const kvList = generateKVList([
        ['ID', user.id],
        ['Tag', user.tag],
        ['Bot', String(user.bot)],
        ['Flags', String(user.flags)],
        ['Guilds', String(user.totalGuilds)],
        ['Bans', String(user.bans.length)],
        ['Connections', String(user.connections.length)],
        ['Premium_Since', user.premiumsince ? new Date(user.premiumsince).toLocaleString() : 'N/A']
      ])

      return context.editOrReply(Markup.codeblock(kvList, { language: 'ml' }))
    }
}
