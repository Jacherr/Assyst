import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    text: string
}

export default class WonkaCommand extends BaseFapiCommand {
    label = 'text'

    name = 'wonka'

    metadata = {
      description: 'Wonka',
      examples: [''],
      usage: '[text]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const res = await this.fapi.wonka(args.text);
      return context.editOrReply({
        file: {
          filename: 'wonka.png',
          value: res
        }
      });
    }
}
