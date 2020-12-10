import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    text: string
}

export default class PresidentialCommand extends BaseFapiCommand {
    label = 'text'

    name = 'presidential'

    metadata = {
      description: 'Presidential',
      examples: [''],
      usage: '[text]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const res = await this.fapi.presidential(args.text);
      return context.editOrReply({
        file: {
          filename: 'presidential.png',
          value: res
        }
      });
    }
}
