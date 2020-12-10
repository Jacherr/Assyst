import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';

export interface CommandArgs {
    text: string
}

export default class RecaptchaCommand extends BaseFapiCommand {
    aliases = ['captcha']

    label = 'text'

    name = 'recaptcha'

    metadata = {
      description: 'Recaptcha',
      examples: [''],
      usage: '[text]'
    }

    async run (context: Command.Context, args: CommandArgs) {
      const res = await this.fapi.recaptcha(args.text);
      return context.editOrReply({
        file: {
          filename: 'recaptcha.png',
          value: res
        }
      });
    }
}
