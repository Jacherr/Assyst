import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import { formatElapsed, elapsed, formatMessageLink, flat } from '../../utils';
import { EmbedColors } from '../../constants';

export default class RemindListCommand extends BaseCommand {
    name = 'remind list'

    metadata = {
        description: 'List your reminders'
    }

    priority = 2

    async run(context: Context) {
        const reminders = await this.assyst.database.getUserReminders(context.userId);
        if (reminders.length === 0) {
            return this.error(context, 'You have no reminders.');
        }
        const formattedReminders = reminders.sort((a, b) => parseInt(a.timestamp) - parseInt(b.timestamp)).map(r => {
            let message;
            if(r.message.length > 100) {
                message = r.message.slice(0, 97) + '...'
            } else {
                message = r.message;
            }
            return `ID: \`${r.message_id}\`\nIn ${formatElapsed(elapsed(parseInt(r.timestamp) - Date.now()))}: [${message}](${formatMessageLink(r.guild_id, r.channel_id, r.message_id)})\n`
        })
        const pagedReminders = flat(formattedReminders, 10);
        const pages = pagedReminders.map(p => Object({
            embed: {
                description: p.join('\n'),
                color: EmbedColors.INFO
            }
        }))
        return this.assyst.paginator.createReactionPaginator({
            pages,
            message: context
        })
    }
}
