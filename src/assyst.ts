import { CommandClient, CommandClientOptions, CommandClientRunOptions, ClusterClient } from 'detritus-client';
import { Context, Command } from 'detritus-client/lib/command';

import { Client } from 'fapi-client';
import { PaginatorCluster } from 'detritus-pagination';

import { tokens, database, prefixOverride, ownerOnly, admins, channelWhitelist, badtranslator } from '../config.json';
import { RequestTypes } from 'detritus-client-rest';
import { Zx8 } from './rest/zx8/zx8';
import { Maryjane } from './rest/maryjane/maryjane';
import { Database } from './database/database';
import { createHash } from 'crypto';
import BadTranslator from './badtranslator.js';

const errorImages: { [key: string]: string } = {
  fcab5a15e2ee436f8694b9777c3cb08b: 'No DNS records.',
  '5991482f1a1d321eea4162044abbfd78': 'The domain does not exist.',
  d4e18d5b499eedb1b3b62d93a669beb8: 'Connection refused.',
  ab341be5ab990e3179bb6c4db954f702: 'Connection reset by peer.'
};

export interface AssystOptions extends CommandClientOptions {
  directory: string
}

export class Assyst extends CommandClient {
  public client!: ClusterClient
  public database: Database
  public directory: string
  public fapi: Client.Client
  public maryjane: Maryjane
  public paginator: PaginatorCluster
  public zx8: Zx8
  public badTranslator?: BadTranslator

  constructor (token: string, options: AssystOptions) {
    super(token, options);

    this.database = new Database(this, database);

    this.directory = options.directory;
    this.fapi = new Client.Client({
      auth: tokens.fapi,
      timeout: 45000
    });

    this.paginator = new PaginatorCluster(this.client, {
      maxTime: 60000,
      pageNumber: true
    });

    this.maryjane = new Maryjane();
    this.zx8 = new Zx8();
  }

  async initBadTranslator() {
    const controller = new BadTranslator(this, badtranslator);
    this.badTranslator = controller;
    await controller.init();
  }

  executeLogWebhook (url: string, options?: string | RequestTypes.ExecuteWebhook) {
    const searchString = 'webhooks';

    const index = url.indexOf(searchString);
    if (index === -1) {
      throw new Error('Invalid Discord webhook URL provided');
    }

    const id = url.slice(index + searchString.length + 1, url.lastIndexOf('/'));
    const token = url.slice(url.lastIndexOf('/') + 1);

    // @ts-ignore
    return this.rest.executeWebhook(id, token, options);
  }

  async resetCommands () {
    this.clear();
    await this.addMultipleIn(this.directory, {
      subdirectories: true
    });
  }

  async run (options?: CommandClientRunOptions) {
    await this.resetCommands();
    return super.run(options);
  }

  async onCommandCheck (context: Context, command: Command) {
    const whitelist = channelWhitelist as string[];
    const whitelistVerifiedAllowed = whitelist.length > 0 ? whitelist.includes(context.channelId) : true;

    if (context.user.isClientOwner) {
      return true;
    } else if (context.inDm ||
      context.user.bot ||
      !whitelistVerifiedAllowed ||
      (ownerOnly && (!context.user.isClientOwner && !admins.includes(context.userId)))) {
      return false;
    }
    return true;
  }

  async onPrefixCheck (context: Context) {
    if (prefixOverride) return prefixOverride;

    const defaultPrefix = 'a-';

    const userId = context.client.userId;
    const prefixes = new Set([`<@${userId}>`, `<@!${userId}>`]);
    const customPrefix = await this.database.fetchGuildPrefix(context.guildId as string);
    if (!customPrefix) {
      await this.database.setGuildPrefix(context.guildId as string, defaultPrefix);
      prefixes.add(defaultPrefix);
      return prefixes;
    }
    prefixes.add(customPrefix);
    return prefixes;
  }

  public async screenshot (url: string, allowNsfw: boolean | undefined, wait: number | undefined) {
    const res = await this.fapi.screenshot(url,
      {
        allowNSFW: allowNsfw,
        wait
      });

    const hash: string = createHash('md5').update(res).digest('hex');
    if (errorImages[hash] !== undefined) throw new Error(errorImages[hash]);

    return res;
  }
}
