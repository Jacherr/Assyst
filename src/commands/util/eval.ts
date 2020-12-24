import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';

import { inspect } from 'util';
import fetch from 'node-fetch';
import { Markup } from 'detritus-client/lib/utils';
import { HttpMethods } from 'fapi-client/JS/src/types';
import { evalUrl } from '../../../config.json';
import { parseCodeblocks } from '../../utils';

export interface CommandArgs {
    code: string
}

export default class EvalCommand extends BaseCommand {
    label = 'code'

    name = 'eval'

    metadata = {
      description: 'Evaluate code',
      examples: ['1+1', 'process.versions'],
      usage: '[code]'
    }

    async run (context: Context, args: CommandArgs) {
      let result = await this.runCode(parseCodeblocks(args.code || 'undefined'));

      try {
        result = JSON.parse(result);
        result = inspect(result, {
          showHidden: true,
          depth: 1
        });
      } catch {}

      context.editOrReply(Markup.codeblock(result || 'undefined', {
        language: 'js',
        limit: 1990
      }));
    }

    async runCode (code: string) {
      const res = await fetch(evalUrl, {
        method: HttpMethods.POST,
        body: JSON.stringify({
          code
        }),
        headers: {
          'content-type': 'application/json'
        }
      });
      if (!res.ok) throw new Error(`${res.status}: ${res.statusText}`);
      return res.json().then(json => json.message);
    }
}
