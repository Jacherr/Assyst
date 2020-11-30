import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    text: string
}

export default class MemorialCommand extends BaseFapiCommand {
    label = 'text'

    name = 'memorial'

    metadata = {
      description: 'Memorial',
      examples: [''],
      usage: '[text]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const res = await this.fapi.memorial(args.text);
      return context.editOrReply({
        file: {
          filename: 'memorial.png',
          value: res
        }
      });
    }
}
