import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import { parseTimestamp, TIME_REGEX, elapsed, formatElapsed } from '../../utils'; 

export interface CommandArgs {
    args: string
}

export default class GifCommand extends BaseCommand {
    label = 'args'

    name = 'remind'

    metadata = {
      description: 'Set a reminder',
      examples: ['2h yeah', '30m 30s yeah']
    }

    async run (context: Context, args: CommandArgs) {
      if (!args.args) {
        return this.error(context, 'Provide a time.');
      }
      const timestamp = parseTimestamp(args.args, 157766400000);
      if(timestamp === 0) {
        return this.error(context, 'No valid time was found. Use the format [val][unit], e.g. 3days 3hours');
      } else if(timestamp < 60000) {
        return this.error(context, 'Time must be at least 1 minute.');
      }

      const userReminders = await this.assyst.database.getUserReminders(context.userId);
      if(userReminders.length > 25) {
        return this.error(context, 'You already have a maximum of 25 reminders set.');
      }

      const endDate = BigInt(Date.now()) + BigInt(timestamp);
      const message = args.args.replace(TIME_REGEX, '');
      if(message.length > 256) {
        return this.error(context, 'The reminder message must be a maximum of 265 characters.');
      }

      await this.assyst.database.setReminder(endDate, message.trim().length > 0 ? message.trim() : '...', context.userId, context.guildId as string, context.channelId, context.messageId);

      return context.editOrReply(`Reminder created for ${formatElapsed(elapsed(timestamp))} from now.`);
    }
}