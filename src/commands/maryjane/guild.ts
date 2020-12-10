import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import { generateKVList, flat } from '../../utils';

import { Markup } from 'detritus-client/lib/utils'

export interface CommandArgs {
  guildId: string
}

export default class MaryjaneGuildCommand extends BaseCommand {
    aliases = ['mj guild']

    name = 'maryjane guild'

    label = 'guildId'

    metadata = {
      description: 'Get a guild from Maryjane API',
      examples: ['178313653177548800'],
      usage: '[guild id]'
    }

    async run (context: Context, args: CommandArgs) {
      const guild = await this.assyst.maryjane.guild(args.guildId || context.guildId as string);

      const kvList = generateKVList([
        ['ID', guild.id],
        ['Name', guild.name],
        ['Description', guild.description !== null ? flat(guild.description.split(' '), 5).map(a => a.join(' ')).join('\n') : 'N/A'],
        ['Members', String(guild.member_count)],
        ['Channels', String(guild.channel_count)],
        ['Emoji', String(guild.emoji_count)],
        ['Roles', String(guild.role_count)],
        ['Owner', guild.ownerid],
        ['Locale', guild.preferred_locale],
        ['Region', guild.region],
        ['Flags', String(guild.flags)],
        ['Invites', guild.invites.length > 0 ? guild.invites.map(i => i.invite).join(', ') : 'None'],
        ['Bans', String(guild.bans.length)]
      ])

      return context.editOrReply(Markup.codeblock(kvList, { language: 'ml' }))
    }
}
