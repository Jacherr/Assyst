import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    text: string
}

export default class SonicCommand extends BaseFapiCommand {
    label = 'text'

    name = 'sonic'

    metadata = {
      description: 'Sonic',
      examples: [''],
      usage: '[text]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const res = await this.fapi.sonic(args.text);
      return context.editOrReply({
        file: {
          filename: 'sonic.png',
          value: res
        }
      });
    }
}
