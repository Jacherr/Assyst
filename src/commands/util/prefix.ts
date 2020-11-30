import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';

export interface CommandArgs {
    prefix: string
}

export default class PrefixCommand extends BaseCommand {
    label = 'prefix'

    name = 'prefix'

    metadata = {
      description: 'View or set this guild\'s prefix',
      examples: ['', '-'],
      usage: '<new prefix>'
    }

    async run (context: Context, args: CommandArgs) {
        if(args.prefix) {
            const isGuildOwner = await this.userOwnsGuild(context);
            if(!isGuildOwner) {
                return this.error(context, 'You need to own the guild to change the prefix.');
            } else if(args.prefix.length > 16) {
                return this.error(context, 'The new prefix needs to be less than 16 characters.');
            }
            await this.assyst.database.editGuildPrefix(context.guildId as string, args.prefix);
            return context.editOrReply(`Prefix changed to \`${args.prefix}\``)
        } else {
            const prefix = await this.assyst.database.fetchGuildPrefix(context.guildId as string);
            return context.editOrReply(`Guild prefix: \`${prefix || 'None'}\``)
        }
    }
}
