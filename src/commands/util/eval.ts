import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';

import { inspect } from 'util';
import fetch from 'node-fetch';
import { Markup } from 'detritus-client/lib/utils';
import { HttpMethods } from 'fapi-client/JS/src/types';
import { evalUrl } from '../../../config.json';
import { parseCodeblocks } from '../../utils';
import { Endpoints } from '../../rest/rest';

export interface CommandArgs {
    code: string,
    ex: boolean
}

export default class EvalCommand extends BaseCommand {
    args = [
      {
        name: 'ex',
        type: Boolean,
        default: false
      }
    ]
    label = 'code'

    name = 'eval'

    metadata = {
      description: 'Evaluate code',
      examples: ['1+1', 'process.versions'],
      usage: '[code]'
    }

    async run (context: Context, args: CommandArgs) {
      let code = parseCodeblocks(args.code) || 'undefined';
      let result = args.ex 
        ? await this.runCodeExperimental(code)
        : await this.runCode(code);

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

    async runCodeExperimental (code: string) {
      return fetch(Endpoints.FAKE_EVAL_EXPERIMENTAL.replace(':code', encodeURIComponent(code)))
        .then(x => x.text());
    }
}
