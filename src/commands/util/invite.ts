import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';

export default class InviteCommand extends BaseCommand {
    aliases = ['inv']

    name = 'invite'

    metadata = {
      description: 'Get the Assyst invite'
    }

    async run (context: Context) {
      const invite = context.client.application?.oauth2UrlFormat({
        scope: 'bot'
      });

      context.editOrReply(`ℹ️ Invite Assyst with this URL: <${invite}>`);
    }
}
