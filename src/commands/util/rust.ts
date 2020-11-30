import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import { parseCodeblocks } from '../../utils';
import { runRustCode } from '../../rest/rest';
import { Markup } from 'detritus-client/lib/utils';

export interface CommandArgs {
    code: string
    channel?: string
    backtrace?: boolean
    cratetype?: string
    edition?: string
    mode?: string
    tests?: boolean
}

export default class RustCommand extends BaseCommand {
    aliases = ['rs']

    args = [
      {
        name: 'channel',
        type: String,
        default: 'stable'
      },
      {
        name: 'backtrace',
        type: Boolean
      },
      {
        name: 'cratetype',
        type: String,
        default: 'bin'
      },
      {
        name: 'edition',
        type: String,
        default: '2018'
      },
      {
        name: 'mode',
        type: String,
        default: 'debug'
      },
      {
        name: 'tests',
        type: Boolean,
        default: false
      }
    ]

    label = 'code'

    name = 'rust'

    metadata = {
      description: 'Execute rust code',
      examples: ['println!("hello")'],
      usage: '[code]'
    }

    async run (context: Context, args: CommandArgs) {
      if (!args.code) {
        return this.error(context, 'Provide some code to execute.');
      }

      const codeToExecute = parseCodeblocks(args.code);
      const result = await runRustCode(codeToExecute, {
        channel: args.channel,
        backtrace: args.backtrace,
        crateType: args.cratetype,
        edition: args.edition,
        mode: args.mode,
        tests: args.tests
      });

      return context.editOrReply(Markup.codeblock(result, {
        language: 'rs'
      }));
    }
}
