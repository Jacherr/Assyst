import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import { EmbedColors } from '../../constants';

export interface CommandArgs {
    id: string
}

export default class CreplsCommand extends BaseCommand {
    name = 'user'

    label = 'id'

    metadata = {
      description: 'Get a user',
      usage: '[id|mention]'
    }

    async run (context: Context, args: CommandArgs) {
        if(!args.id) {
            return this.error(context, 'Provide a user id or mention.')
        }
        const id = this.parseMentionOrId(args.id);
        const [discordUser, maryjaneUser] = await Promise.all([
            context.rest.fetchUser(id),
            this.assyst.maryjane.user(id)
        ])

        return context.editOrReply({ embed: { description: [
            `**ID:** ${discordUser.id}`,
            `**Tag:** ${discordUser.username}#${discordUser.discriminator}`,
            `**Created At:** ${discordUser.createdAt.toLocaleString()}`,
            `**Bot:** ${discordUser.bot}`,
            `**Premium Since:** ${maryjaneUser.premiumsince}`
        ].join('\n'), color: EmbedColors.INFO}})
    }
}
