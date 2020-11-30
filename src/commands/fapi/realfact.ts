import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    text: string
}

export default class RealFactCommand extends BaseFapiCommand {
    aliases = ['fact']

    label = 'text'

    name = 'realfact'

    metadata = {
      description: 'Real Fact',
      examples: [''],
      usage: '[text]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const res = await this.fapi.realFact(args.text);
      return context.editOrReply({
        file: {
          filename: 'realfact.png',
          value: res
        }
      });
    }
}
