import { CommandClient, Command } from 'detritus-client';
import { Message, MessageEmbed, Attachment } from 'detritus-client/lib/structures';

import { BaseCommand } from './basecommand';
import { Assyst } from '../assyst';
import { Context } from 'detritus-client/lib/command';
import { parseCodeblocks } from '../utils';

export class BaseFapiCommand extends BaseCommand {
  constructor (commandClient: CommandClient, options: Partial<Command.CommandOptions>) {
    super(commandClient, Object.assign({
      name: '',
      ratelimits: [
        { duration: 5000, limit: 5, type: 'guild' },
        { duration: 2000, limit: 1, type: 'channel' }
      ]
    }, options));
  }

  get fapi () {
    return (this.commandClient as Assyst).fapi;
  }

  async onBeforeRun (context: Command.Context): Promise<boolean> {
    await context.triggerTyping();
    return true;
  }

  public parseImageScriptArgs (args: string): [string, string] {
    const indexOfWhitespace = args.search(/\s/);
    if (indexOfWhitespace === -1) return [args, ''];
    const firstArg = args.slice(0, indexOfWhitespace);
    const restArgs = args.slice(indexOfWhitespace);
    return [firstArg, restArgs.trim()];
  }

  public async injectImageScriptPackages (script: string) {
    const directive = '///USE';
    const lines = script.split('\n');
    let index = -1;
    const importedPackages: string[] = [];
    for (const line of lines) {
      index++;
      if (line.startsWith(directive)) {
        const packageName = line.split(' ')[1].trim();
        if (!packageName || importedPackages.includes(packageName)) continue;
        const isPackage = await this.assyst.database.fetchImageScriptPackage(packageName);
        if (!isPackage) continue;
        lines[index] = `(() => { 
          try {
            ${isPackage.content} 
          } catch(_packageError) {
            throw new Error('Package \\'${isPackage.name}\\' threw an error: ' + (_packageError.message || _packageError))
          }
        })();`;
        importedPackages.push(isPackage.name);
      }
    }
    return lines.join('\n');
  }

  public async loadCode (context: Context, messageContent: string) {
    let code;

    if (context.message.attachments.first()) {
      const attachment = context.message.attachments.first() as Attachment;
      if (!attachment.url) code = parseCodeblocks(messageContent);
      const data = await context.rest.request(attachment.url as string);
      code = data;
    } else {
      code = parseCodeblocks(messageContent);
    }
    return code;
  }
}
