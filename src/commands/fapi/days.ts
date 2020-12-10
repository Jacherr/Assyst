import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    text: string
}

export default class DaysCommand extends BaseFapiCommand {
    label = 'text'

    name = 'days'

    metadata = {
      description: 'Days',
      examples: [''],
      usage: '[text]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const res = await this.fapi.days(args.text);
      return context.editOrReply({
        file: {
          filename: 'days.png',
          value: res
        }
      });
    }
}
