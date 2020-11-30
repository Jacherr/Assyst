import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import fetch from 'node-fetch';

const CREPLS_ID = '302394290368151553';

export default class CreplsCommand extends BaseCommand {
    name = 'crepls'

    metadata = {
      description: 'Get crepls'
    }

    async run (context: Context) {
        const crepls = await context.rest.fetchUser(CREPLS_ID);
        const creplsAvatar = crepls.avatarUrl;
        const buffer = await fetch(creplsAvatar).then(x => x.buffer());

        return context.editOrReply({
            content: 'this is crepls',
            file: {
                filename: 'crepls.png',
                value: buffer
            }
        })
    }
}
