/* eslint-disable no-eval */
import { Command } from 'detritus-client';
import { execSync } from 'child_process';

import { BaseAdminCommand } from '../baseadmincommand';
import { Markup } from 'detritus-client/lib/utils';

export interface CommandArgs {
  query: string
}

export default class UpdateCommand extends BaseAdminCommand {
  name = 'update'

  metadata = {
    description: 'Update the bot',
    examples: [''],
    usage: ''
  }

  async run(context: Command.Context, args: CommandArgs) {
    await context.triggerTyping();
    const out = execSync('git pull && tsc');
    await context.reply(Markup.codeblock(out.toString(), {
      language: 'bash'
    }));
    delete require.cache;
    this.assyst.resetCommands();
  }
}
