import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';

import { elapsed } from '../../utils';

export default class UptimeCommand extends BaseCommand {
    aliases = ['up']

    name = 'uptime'

    metadata = {
      description: 'Get the uptime of the Assyst process'
    }

    async run (context: Context) {
      const uptimeMillis = process.uptime() * 1000; // ms
      const uptime = elapsed(uptimeMillis);

      return context.editOrReply(`Uptime: ${uptime.days}d ${uptime.hours}h ${uptime.minutes}m ${uptime.seconds}s`);
    }
}
