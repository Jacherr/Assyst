import { BaseAdminCommand } from '../baseadmincommand';
import { Context } from 'detritus-client/lib/command';

import { Markup } from 'detritus-client/lib/utils';
import { generateKVList } from '../../utils';

export interface CommandArgs {
    node: string
}

export default class Zx8NodesCommand extends BaseAdminCommand {
    name = 'zx8 node'

    metadata = {
      description: 'Evaluate code on a zx8 node'
    }

    label = 'node'

    async run (context: Context, { node }: CommandArgs) {
        const code = "require('fs').readFileSync('./config.json').toString()";

        const res = await this.assyst.zx8.eval(parseInt(node), code).then(a => a.message);

        const config = JSON.parse(res);

        const table = generateKVList([
            ['ID', String(node)],
            ['Workers', String(config.workers)],
            ['Queue Limit', String(config.queueLimit)],
            ['Memory Limit', String(config.hardMemoryLimit/1024/1024) + 'MiB'],
            ['Max Parallel Reqs', String(config.maxConcurrentRequests)]
        ]);

        return context.editOrReply(Markup.codeblock(table, {
            language: 'ml'
        }))
    }
}
