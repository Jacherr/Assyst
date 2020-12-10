import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    text: string
}

export default class SimpsonsDisabledCommand extends BaseFapiCommand {
    aliases = ['simpsons']

    label = 'text'

    name = 'simpsonsdisabled'

    metadata = {
      description: 'Simpsons Disabled',
      examples: [''],
      usage: '[text]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const res = await this.fapi.simpsonsDisabled(args.text);
      return context.editOrReply({
        file: {
          filename: 'simpsonsdisabled.png',
          value: res
        }
      });
    }
}
