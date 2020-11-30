import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';

export interface CommandArgs {
    host: string
}

export default class PingCommand extends BaseCommand {
    aliases = ['pong']

    label = 'host'

    name = 'ping'

    metadata = {
      description: 'Ping the Discord REST and WebSocket APIs'
    }

    async run (context: Context, _args: CommandArgs) {
      const { gateway, rest } = await context.client.ping();

      return context.editOrReply(`Pong! REST: ${rest}ms, WS: ${gateway}ms`);
    }
}
