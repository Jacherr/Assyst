import { BaseFapiCommand } from '../basefapicommand';
import { Command } from 'detritus-client';
import { EmbedColors } from '../../constants'

export interface CommandArgs {
    query: string
}

export default class DuckDuckGoCommand extends BaseFapiCommand {
    label = 'query'

    name = 'duckduckgo'

    aliases = ['search', 'g', 'ddg']

    metadata = {
        description: 'Search Duck Duck Go',
        examples: ['hat'],
        usage: '[query]'
    }

    async run(context: Command.Context, args: CommandArgs) {
        let result = await this.fapi.duckDuckGo(args.query);

        if(result.results.length === 0) {
            return this.error(context, 'No results found');
        }

        let format = result.results.map(r => `[${r.title}](${r.link})`);

        return context.editOrReply({
            embed: {
                title: `Search results: ${args.query}`,
                description: format.join('\n'),
                color: EmbedColors.INFO
            }
        })
    }
}
