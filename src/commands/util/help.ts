import { BaseCommand } from '../basecommand';
import { Context } from 'detritus-client/lib/command';
import { EmbedColors } from '../../constants';
import { Markup } from 'detritus-client/lib/utils';

export interface Category {
  name: string,
  commands: string[]
}

export interface CommandArgs {
  command: string
}

export default class HelpCommand extends BaseCommand {
  label = 'command';

  name = 'help';
  
  metadata = {
    description: 'Get Assyst full or specific command help',
    examples: ['', 'ping'],
    usage: '<command>'
  }

  async run(context: Context, args: CommandArgs) {
    if (!args.command) {
      const categories: Category[] = [];
      for (const command of this.assyst.commands) {
        if ((await command.onBefore?.(context)) === true) {
          const categoryNames = categories.map(c => c.name);
          const commandCategory = this.getCategory(command._file as string);
          if (!categoryNames.includes(commandCategory)) {
            categories.push({
              name: commandCategory,
              commands: [command.name]
            });
          } else {
            const category = categories.find(c => c.name === commandCategory) as Category;
            category.commands.push(command.name);
          }
        }
      }

      const fields = categories.map(c => ({
        name: c.name,
        value: c.commands.map(cmd => `\`${cmd}\``).join(','),
        inline: false
      }));

      return context.editOrReply({
        embed: {
          fields,
          color: EmbedColors.INFO,
          title: 'Assyst Commands'
        }
      });
    } else {
      const command = this.assyst.commands.find((c) => {
        return c.name === args.command || c.aliases.includes(args.command);
      });
      if (!command) {
        return this.error(context, 'No command with this name or alias exists.');
      }
      return context.editOrReply({
        embed: {
          color: EmbedColors.INFO,
          description: command.metadata.description || 'No description found...',
          title: `Command: ${command.name}`,
          fields: [
            {
              name: 'Usage',
              value: (() => {
                const usage = command.metadata.usage;
                if (!usage) return Markup.codeblock(context.prefix + command.name);
                return Markup.codeblock(context.prefix + command.name + ' ' + usage);
              })(),
              inline: false
            },
            {
              name: 'Examples',
              value: (() => {
                const examples = command.metadata.examples;
                if (!examples) return Markup.codeblock(context.prefix + command.name);
                return Markup.codeblock(examples.map((e: string) => `${context.prefix}${command.name} ${e}`).join('\n'));
              })()
            }
          ]
        }
      });
    }
  }

  public getCategory(path: string) {
    const parts = path.replace(/\\/g, '/').split('/');
    const category = parts[parts.length - 2];
    return category;
  }
}
