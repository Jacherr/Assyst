import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';
import { parseCodeblocks } from '../../utils';
import { Markup } from 'detritus-client/lib/utils';

export interface CommandArgs {
    code: string,
    lang: string
}

export default class RextesterCommand extends BaseFapiCommand {
    label = 'code'

    name = 'rextester'

    aliases = ['rex']

    args = [
        {
          name: 'lang',
          type: String,
          default: 'node'
        }
    ]

    metadata = {
      description: 'Run code on rextester',
      examples: ['console.log(1)', 'print(1) -lang py'],
      usage: '[code] <-lang language>'
    }

    async run (context: Command.Context, args: CommandArgs) {
        const code = parseCodeblocks(args.code);

        let result = await this.fapi.rexTester(args.lang, code).then((a: string | undefined) => a?.toString());

        return context.reply(Markup.codeblock(result ?? 'Empty response', {
            language: args.lang
        }));
    }
}
