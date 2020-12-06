import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';

export interface CommandArgs {
    undo: string
}

export default class UndoCommand extends BaseCommand {

    name = 'clean'

    metadata = {
        description: 'Clean recent commands'
    }

    async onBefore(context: Context) {
        const MANAGE_MESSAGES_BITFLAG = 8192;
        const ADMIN_BITFLAG = 8;

        const [member, roles] = await Promise.all([
            context.rest.fetchGuildMember(context.guildId as string, context.userId),
            context.rest.fetchGuildRoles(context.guildId as string)
          ]);
        
          const userRoles = roles.filter(x => member.roles.has(x.id));
        
          const permissions = userRoles.reduce((p, c) => p | c.permissions, 0);

          if((permissions & MANAGE_MESSAGES_BITFLAG) === MANAGE_MESSAGES_BITFLAG || (permissions & ADMIN_BITFLAG) === ADMIN_BITFLAG) return super.onBefore(context);
          return false;
    }

    async run(context: Context, args: CommandArgs) {
        let amount = 100;
        const deleteTimeout = 500;

        if (args && args.undo && !isNaN(parseInt(args.undo))) {
            amount = parseInt(args.undo);
            if (amount > 2000) amount = 2000;
            else if (amount < 1) amount = 1;
        }

        const allRecentInvocations = this.assyst.replies.filter(r => r.context.channelId === context.channelId && !r.reply.deleted);
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