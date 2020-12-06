import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';

export interface CommandArgs {
    undo: string
}

export default class UndoCommand extends BaseCommand {

    name = 'undo'

    metadata = {
        description: 'Undo recent commands'
    }

    async run(context: Context, args: CommandArgs) {
        let amount = 1;
        const deleteTimeout = 500;

        if (args && args.undo && !isNaN(parseInt(args.undo))) {
            amount = parseInt(args.undo);
            if (amount > 5) amount = 5;
            else if (amount < 1) amount = 1;
        }

        const allRecentInvocations = this.assyst.replies.filter(r => r.context.userId === context.userId && r.context.channelId === context.channelId && !r.reply.deleted);
        const recentInvocations = allRecentInvocations.slice(allRecentInvocations.length - amount);
        const messageIds = [...recentInvocations.map(r => r.reply.id), ...recentInvocations.map(r => r.context.message.id)];

        try {
            await context.rest.bulkDeleteMessages(context.channelId, messageIds);
            if (recentInvocations.length === 0) {
                return this.error(context, 'No recent commands found.').then((res) => {
                    setTimeout(async () => await context.rest.bulkDeleteMessages(context.channelId, [res.id, context.messageId]).catch(() => 1), deleteTimeout);
                });
            }
        } catch (e) {
            return context.editOrReply(e.message);
        }

        return context.editOrReply(`${messageIds.length} messages deleted`).then((res) => {
            setTimeout(async () => await context.rest.bulkDeleteMessages(context.channelId, [res.id, context.messageId]), deleteTimeout);
        });
    }
}