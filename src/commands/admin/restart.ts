import { Command } from 'detritus-client';
import { BaseAdminCommand } from '../baseadmincommand';

export default class RestartCommand extends BaseAdminCommand {
  name = 'restart'

  metadata = {
    description: 'Restart the bot'
  }

  async run(context: Command.Context) {
    const statusCode = -1;
    await context.editOrReply('restarting !!!!🕴️')
    process.exit(statusCode);
  }
}
