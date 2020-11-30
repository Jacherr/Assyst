import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import { User } from 'detritus-client/lib/structures';
import fetch from 'node-fetch';

export interface CommandArgs {
    user: string
}

export default class AvatarCommand extends BaseCommand {
    aliases = ['av']

    label = 'user'

    name = 'avatar'

    metadata = {
      description: 'Get a user\'s avatar'
    }

    async run (context: Context, args: CommandArgs) {
        let user: User;
        if(args.user) {
            let id = this.parseMentionOrId(args.user.split(/\s/g)[0]);
            try {
                user = await context.rest.fetchUser(id);
            } catch {
                return this.error(context, 'User not found')
            }
        } else {
            user = context.user;
        }

        let avatar = user.avatarUrl;
        let ext = avatar.split('.').pop();
        let buffer = await fetch(avatar).then(x => x.buffer());

        return context.editOrReply({
            file: {
                filename: `avatar.${ext}`,
                value: buffer
            }
        })
    }
}
