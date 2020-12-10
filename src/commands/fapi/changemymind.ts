import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    text: string
}

export default class ChangeMyMindCommand extends BaseFapiCommand {
    aliases = ['cmm']

    label = 'text'

    name = 'changemymind'

    metadata = {
      description: 'Change My Mind',
      examples: [''],
      usage: '[text]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const res = await this.fapi.changeMyMind(args.text);
      return context.editOrReply({
        file: {
          filename: 'changemymind.png',
          value: res
        }
      });
    }
}
