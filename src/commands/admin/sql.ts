/* eslint-disable no-eval */
import { Command } from 'detritus-client';
import { Markup } from 'detritus-client/lib/utils';
import { inspect } from 'util';

import { BaseAdminCommand } from '../baseadmincommand';

export interface CommandArgs {
  query: string
}

export default class SQLCommand extends BaseAdminCommand {
  label = 'query'

  name = 'sql'

  metadata = {
    description: 'Execute SQL',
    examples: ['select now(*)'],
    usage: '[query]'
  }

  async run(context: Command.Context, args: CommandArgs) {
    const result = await this.assyst.database.sql(args.query).then(res => res.rows);

    return context.editOrReply(Markup.codeblock(inspect(result), {
      language: 'js',
      limit: 1990
    }));
  }
}
