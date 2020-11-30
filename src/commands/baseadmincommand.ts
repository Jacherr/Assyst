import { Command, CommandClient } from 'detritus-client';

import { BaseCommand } from './basecommand';

import { admins } from '../../config.json';

export class BaseAdminCommand extends BaseCommand {
  constructor (commandClient: CommandClient, options: Partial<Command.CommandOptions>) {
    super(commandClient, Object.assign({
      name: '',
      ratelimits: []
    }, options));
  }

  async onBefore (context: Command.Context): Promise<boolean> {
    super.onBefore(context);
    if (!context.user.isClientOwner && !admins.includes(context.userId)) {
      return false;
    }
    return true;
  }
}
