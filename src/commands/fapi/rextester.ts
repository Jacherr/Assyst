import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';
import { parseCodeblocks } from '../../utils';
import { Markup } from 'detritus-client/lib/utils';
import { runCode } from '../../rest/rest';

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

        let result = await runCode(args.lang, code);

        return context.reply(Markup.codeblock(result, { language: args.lang }));
    }
}
