import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    text: string
}

export default class GrillCommand extends BaseFapiCommand {
    label = 'text'

    name = 'grill'

    metadata = {
      description: 'Grill',
      examples: [''],
      usage: '[text]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const res = await this.fapi.grill(args.text);
      return context.editOrReply({
        file: {
          filename: 'grill.png',
          value: res
        }
      });
    }
}
