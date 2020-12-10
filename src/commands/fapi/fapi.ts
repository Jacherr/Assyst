import { BaseFapiCommand } from '../basefapicommand';
import { Context } from 'detritus-client/lib/command';
import { generateKVList } from '../../utils';
import { ReturnHeaders } from 'fapi-client/JS/src/types';
import { Markup } from 'detritus-client/lib/utils';

export default class FapiCommand extends BaseFapiCommand {
    name = 'fapi'

    metadata = {
      description: 'Get fAPI info for Assyst'
    }

    async run (context: Context) {
      const limits = this.fapi.ratelimits;
      const table = generateKVList([
        [
            ReturnHeaders.RATELIMIT_LIMIT as string,
            (() => {
              const result = limits[ReturnHeaders.RATELIMIT_LIMIT]?.toString();
              if (result) return parseInt(result).toLocaleString() + 'ms';
              return 'none';
            })()
        ],
        [
            ReturnHeaders.RATELIMIT_REMAINING as string,
            (() => {
              const result = limits[ReturnHeaders.RATELIMIT_REMAINING];
              if (result) return Math.floor(result).toLocaleString() + 'ms';
              return 'none';
            })()
        ],
        [
            ReturnHeaders.RATELIMIT_RESET as string,
            (() => {
              const result = limits[ReturnHeaders.RATELIMIT_RESET]?.toString();
              if (result) return parseInt(result).toLocaleString() + 's';
              return 'none';
            })()
        ],
        [
          'timeout',
          this.fapi.timeout.toString() + 'ms'
        ],
        [
          'ping',
          (await this.fapi.ping()) + 'ms'
        ]
      ]);

      return context.editOrReply(Markup.codeblock(table, {
        language: 'ml'
      }));
    }
}
