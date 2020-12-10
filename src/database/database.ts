import { Pool, QueryResult } from 'pg';
import { BaseCollection, BaseCollectionOptions } from 'detritus-client/lib/collections';

import { Assyst } from '../assyst';
import { ImageScriptTag, ImageScriptPackage } from './types';

export interface DatabaseAuth {
    host: string,
    user: string,
    password: string,
    database: string,
    port: number
}

export enum TableNames {
    IMAGESCRIPT_PACKAGES = 'is_packages',
    IMAGESCRIPT_TAGS = 'is_tags',
    PREFIXES = 'prefixes'
}

export class Database {
    static cacheOptions: BaseCollectionOptions = {
      expire: 60000,
      limit: 100
    }

    private assyst: Assyst;
    private db: Pool

    public imageScriptPackages: BaseCollection<string, ImageScriptPackage> = new BaseCollection<string, ImageScriptPackage>(Database.cacheOptions)
    public imageScriptTags: BaseCollection<string, ImageScriptTag> = new BaseCollection<string, ImageScriptTag>(Database.cacheOptions)

    public guildPrefixes: BaseCollection<string, string> = new BaseCollection<string, string>(Database.cacheOptions)

    constructor (assyst: Assyst, db: DatabaseAuth) {
      this.assyst = assyst;
      this.db = new Pool(db);
    }

    public async sql (query: string, values?: any[]): Promise<QueryResult> {
      return new Promise((resolve, reject) => {
        this.db.query(query, values || [], (err: any, res: any) => {
          if (err) reject(err);
          else resolve(res);
        });
      });
    }

    public async getDatabaseSize(): Promise<string> {
      return this.sql(`select pg_size_pretty(pg_database_size('assyst'))`).then(r => r.rows[0]['pg_size_pretty']);
    }

    public async createImageScriptPackage (name: string, content: string, owner: string): Promise<void> {
      await this.sql(`insert into ${TableNames.IMAGESCRIPT_PACKAGES}(name, content, owner) values($1, $2, $3)`, [name, content, owner]);
      this.imageScriptPackages.set(name, {
        name,
        content,
        owner
      });
    }

    public async createImageScriptTag (name: string, content: string, owner: string): Promise<void> {
      await this.sql(`insert into ${TableNames.IMAGESCRIPT_TAGS}(name, content, owner, uses) values($1, $2, $3, 0)`, [name, content, owner]);
      this.imageScriptTags.set(name, {
        name,
        content,
        owner,
        uses: 0
      });
    }

    public async deleteImageScriptPackage (name: string): Promise<void> {
      await this.sql(`delete from ${TableNames.IMAGESCRIPT_PACKAGES} where name = $1`, [name]);
      this.imageScriptPackages.delete(name);
    }

    public async deleteImageScriptTag (name: string): Promise<void> {
      await this.sql(`delete from ${TableNames.IMAGESCRIPT_TAGS} where name = $1`, [name]);
      this.imageScriptTags.delete(name);
    }

    public async editImageScriptPackage (name: string, content: string, owner: string): Promise<void> {
      await this.sql(`update ${TableNames.IMAGESCRIPT_PACKAGES} set content = $1 where name = $2`, [content, name]);
      this.imageScriptPackages.set(name, {
        name,
        content,
        owner
      });
    }

    public async editImageScriptTag (name: string, content: string, owner: string, uses: number): Promise<void> {
      await this.sql(`update ${TableNames.IMAGESCRIPT_TAGS} set content = $1 where name = $2`, [content, name]);
      this.imageScriptTags.set(name, {
        name,
        content,
        owner,
        uses
      });
    }

    public async fetchImageScriptPackage (name: string): Promise<ImageScriptPackage | undefined> {
      let isPackage = this.imageScriptPackages.get(name);
      if (isPackage) return isPackage;
      isPackage = await this.sql(`select * from ${TableNames.IMAGESCRIPT_PACKAGES} where name = $1`, [name]).then(r => r.rows[0]);
      if (isPackage) this.imageScriptPackages.set(name, isPackage);
      return isPackage;
    }

    public async fetchImageScriptTag (name: string): Promise<ImageScriptTag | undefined> {
      let tag = this.imageScriptTags.get(name);
      if (tag) return tag;
      tag = await this.sql(`select * from ${TableNames.IMAGESCRIPT_TAGS} where name = $1`, [name]).then(res => res.rows[0]);
      if (tag) this.imageScriptTags.set(name, tag);
      return tag;
    }

    public async fetchUserImageScriptTags (owner: string): Promise<ImageScriptTag[]> {
      const tags = await this.sql(`select * from ${TableNames.IMAGESCRIPT_TAGS} where owner = $1 order by uses desc`, [owner]).then(res => res.rows);
      for (const tag of tags) {
        this.imageScriptTags.set(tag.name, tag);
      }
      return tags;
    }

    public async fetchTopImageScriptTags (): Promise<ImageScriptTag[]> {
      const tags = await this.sql(`select * from ${TableNames.IMAGESCRIPT_TAGS} order by uses desc limit 10`).then(r => r.rows);
      for (const tag of tags) {
        this.imageScriptTags.set(tag.name, tag);
      }
      return tags;
    }

    public async fetchGuildPrefix (guildId: string): Promise<string | undefined> {
      let prefix = this.guildPrefixes.get(guildId);
      if (prefix) return prefix;
      prefix = await this.sql(`select prefix from ${TableNames.PREFIXES} where guild = $1`, [guildId]).then(res => res.rows[0] ? res.rows[0].prefix : undefined);
      if (prefix) this.guildPrefixes.set(guildId, prefix);
      return prefix;
    }

    public async setGuildPrefix (guildId: string, prefix: string): Promise<void> {
      this.guildPrefixes.set(guildId, prefix);
      this.sql(`insert into ${TableNames.PREFIXES}(guild, prefix) values($1, $2)`, [guildId, prefix]);
    }

    public async editGuildPrefix(guildId: string, prefix: string): Promise<void> {
      this.guildPrefixes.set(guildId, prefix);
      await this.sql(`update ${TableNames.PREFIXES} set prefix = $1 where guild = $2`, [prefix, guildId]);
    }
}
