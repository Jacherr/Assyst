import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    text: string
}

export default class ThonkifyCommand extends BaseFapiCommand {
    aliases = ['thonk']

    label = 'text'

    name = 'thonkify'

    metadata = {
      description: 'Thonkify',
      examples: [''],
      usage: '[text]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const res = await this.fapi.thonkify(args.text);
      return context.editOrReply({
        file: {
          filename: 'thonkify.png',
          value: res
        }
      });
    }
}
