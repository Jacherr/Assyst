import { BaseAdminCommand } from '../baseadmincommand';
import { Context } from 'detritus-client/lib/command';

import { parseCodeblocks } from '../../utils';
import { Markup } from 'detritus-client/lib/utils';

export interface CommandArgs {
    args: string
}

export default class Zx8NodesCommand extends BaseAdminCommand {
    aliases = ['zx8 rpc']

    name = 'zx8 eval'

    metadata = {
      description: 'Evaluate code on a zx8 node'
    }

    label = 'args'

    async run (context: Context, { args }: CommandArgs) {
        const splitArgs = args.split(/ +/g);
        const node = splitArgs[0];
        const code = parseCodeblocks(splitArgs.slice(1).join(' ')).trim();

        const res = await this.assyst.zx8.eval(parseInt(node), code).then(a => a.message);

        return context.editOrReply(Markup.codeblock(res, {
            language: 'js'
        }))
    }
}
