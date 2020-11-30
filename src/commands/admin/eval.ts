/* eslint-disable no-eval */
import { Command } from 'detritus-client';
import { Markup } from 'detritus-client/lib/utils';
import { inspect } from 'util';

import { BaseAdminCommand } from '../baseadmincommand';
import { parseCodeblocks } from '../../utils';
import { tokens } from '../../../config.json';

const tokensList = Object.values(tokens).join('|');

// eslint-disable-next-line no-useless-escape
const TOKEN_REGEX: RegExp = new RegExp(tokensList.replace(/([\.\{\}\(\)\*\+\-\=\!\?\^\$])/g, '\\$1'), 'g');
const TIMEOUT = 10000;

const fns: any[] = [];

const suite = {
  add(data: any) {
    fns.push(data);
    return this;
  },
  run(data: any) {
    const results = [];
    for (const test of fns) {
      const before = Date.now();
      for (let i = 0; i < data.iterations; ++i) test.fn(i);
      results.push({ name: test.name, time: Date.now() - before });
    }
    return results;
  }
};

function bm(...funcs: any[]) {
  let i = 0;
  let out;
  for (const arg of funcs) {
    if (typeof arg === 'function') {
      suite.add({ name: i, fn: arg });
    } else {
      out = suite.run({ iterations: parseInt(arg, 10) });
      break;
    }
    i++;
  }
  fns.splice(0, fns.length);
  return out?.map(i => i.time + 'ms');
}

export interface CommandArgs {
  code: string,
  noreply: boolean,
  depth: number,
  attach: boolean,
  async: boolean
}

export default class EvalCommand extends BaseAdminCommand {
  //@ts-ignore
  args = [
    {
      name: 'async',
      type: Boolean,
      default: false
    },
    {
      name: 'attach',
      type: Boolean,
      default: false
    },
    {
      name: 'depth',
      default: '0',
      type: Number
    },
    {
      name: 'noreply',
      type: Boolean,
      default: false
    }
  ]

  label = 'code'

  name = 'e'

  metadata = {
    description: 'Evaluate JavaScript',
    examples: ['1+1'],
    usage: '[code]'
  }

  async run(context: Command.Context, args: CommandArgs) {
    let evaled: any;
    const code = parseCodeblocks(args.code);

    try {
      if (!args.async) {
        evaled = await Promise.resolve(eval(code));
      } else {
        evaled = await Promise.resolve(eval(`(async () => {\n${code}\n})()`));
      }
    } catch (e) {
      return context.editOrReply(Markup.codeblock(e.message || e.stack || e.toString(), { limit: 1990, language: 'js' }));
    }

    if (args.attach && !args.noreply) {
      let extension = 'txt';

      if (Buffer.isBuffer(evaled)) extension = 'png';
      else if (typeof evaled === 'object') {
        evaled = inspect(evaled, { depth: args.depth, showHidden: true });
      } else {
        evaled = String(evaled);
      }

      if (typeof evaled === 'string') evaled = evaled.replace(TOKEN_REGEX, '');
      return context.editOrReply({ file: { value: evaled, filename: `eval.${extension}` } });
    } else if (!args.noreply) {
      if (typeof evaled === 'object') {
        evaled = inspect(evaled, { depth: args.depth, showHidden: true });
      } else {
        evaled = String(evaled);
      }

      evaled = evaled.replace(TOKEN_REGEX, '');

      return context.editOrReply(Markup.codeblock(evaled, {
        language: 'js',
        limit: 1990
      }));
    }
  }
}
