/* eslint-disable no-eval */
import { Command } from 'detritus-client';
import { exec } from 'child_process';

import { BaseAdminCommand } from '../baseadmincommand';
import { Markup } from 'detritus-client/lib/utils';

export interface CommandArgs {
    script: string
}

export default class ExecCommand extends BaseAdminCommand {
    name = 'ex'

    label = 'script'

    metadata = {
        description: 'Execute shell commands',
        examples: ['ps aux'],
        usage: '[script]'
    }

    async run(context: Command.Context, args: CommandArgs) {
        await context.triggerTyping();
        const result = await this.exec(args.script);
        const output = Markup.codeblock(result, { language: 'sh' });
        return context.editOrReply(output);
    }

    async exec(script: string): Promise<string> {
        return Promise.race([
            new Promise((resolve, reject) => {
                exec(script, (error, stdout, stderr) => {
                    if (error) reject(new Error(error.message));
                    if (stderr && !stdout) reject(new Error(stderr));
                    resolve(stdout);
                })
            }),

            new Promise((_, r) => setTimeout(() => r('Timed out after 10000ms'), 10000)) as Promise<string>
        ]) as Promise<string>
    }
}
