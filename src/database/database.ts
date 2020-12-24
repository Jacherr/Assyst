import { Pool, QueryResult } from 'pg';
import { BaseCollection, BaseCollectionOptions } from 'detritus-client/lib/collections';

import { Assyst } from '../assyst';
import { ImageScriptTag, ImageScriptPackage, Reminder } from './types';

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
    PREFIXES = 'prefixes',
    REMINDERS = 'reminders'
}

export class Database {
    private assyst: Assyst;
    private db: Pool

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
    }

    public async createImageScriptTag (name: string, content: string, owner: string): Promise<void> {
      await this.sql(`insert into ${TableNames.IMAGESCRIPT_TAGS}(name, content, owner, uses) values($1, $2, $3, 0)`, [name, content, owner]);
    }

    public async deleteImageScriptPackage (name: string): Promise<void> {
      await this.sql(`delete from ${TableNames.IMAGESCRIPT_PACKAGES} where name = $1`, [name]);
    }

    public async deleteImageScriptTag (name: string): Promise<void> {
      await this.sql(`delete from ${TableNames.IMAGESCRIPT_TAGS} where name = $1`, [name]);
    }

    public async editImageScriptPackage (name: string, content: string, owner: string): Promise<void> {
      await this.sql(`update ${TableNames.IMAGESCRIPT_PACKAGES} set content = $1 where name = $2`, [content, name]);
    }

    public async editImageScriptTag (name: string, content: string, owner: string, uses: number): Promise<void> {
      await this.sql(`update ${TableNames.IMAGESCRIPT_TAGS} set content = $1 where name = $2`, [content, name]);
    }

    public async fetchImageScriptPackage (name: string): Promise<ImageScriptPackage | undefined> {
      return this.sql(`select * from ${TableNames.IMAGESCRIPT_PACKAGES} where name = $1`, [name]).then(r => r.rows[0]);
    }

    public async fetchImageScriptTag (name: string): Promise<ImageScriptTag | undefined> {
      return this.sql(`select * from ${TableNames.IMAGESCRIPT_TAGS} where name = $1`, [name]).then(res => res.rows[0]);
    }

    public async fetchUserImageScriptTags (owner: string): Promise<ImageScriptTag[]> {
      return await this.sql(`select * from ${TableNames.IMAGESCRIPT_TAGS} where owner = $1 order by uses desc`, [owner]).then(res => res.rows);
    }

    public async fetchTopImageScriptTags (): Promise<ImageScriptTag[]> {
      return this.sql(`select * from ${TableNames.IMAGESCRIPT_TAGS} order by uses desc limit 10`).then(r => r.rows);
    }

    public setImageScriptTagUses(name: string, uses: number) {
      return this.sql(`update ${TableNames.IMAGESCRIPT_TAGS} set uses = $1 where name = $2`, [uses, name]);
    }

    public async fetchGuildPrefix (guildId: string): Promise<string | undefined> {
      return this.sql(`select prefix from ${TableNames.PREFIXES} where guild = $1`, [guildId]).then(res => res.rows[0] ? res.rows[0].prefix : undefined);
    }

    public async setGuildPrefix (guildId: string, prefix: string): Promise<void> {
      this.sql(`insert into ${TableNames.PREFIXES}(guild, prefix) values($1, $2)`, [guildId, prefix]);
    }

    public async editGuildPrefix(guildId: string, prefix: string): Promise<void> {
      await this.sql(`update ${TableNames.PREFIXES} set prefix = $1 where guild = $2`, [prefix, guildId]);
    }

    public setReminder(timestamp: BigInt, message: string, userId: string, guildId: string, channelId: string, messageId: string) {
      return this.sql(`insert into ${TableNames.REMINDERS}(user_id, timestamp, guild_id, channel_id, message_id, message) values($1, $2, $3, $4, $5, $6)`, [userId, timestamp, guildId, channelId, messageId, message])
    }

    public getUserReminders(userId: string): Promise<Reminder[]> {
      return this.sql(`select * from ${TableNames.REMINDERS} where user_id = $1`, [userId]).then(r => r.rows);
    }

    public getAllReminders(): Promise<Reminder[]> {
      return this.sql(`select * from ${TableNames.REMINDERS}`).then(r => r.rows);
    }

    public deleteReminder(messageId: string) {
      return this.sql(`delete from ${TableNames.REMINDERS} where message_id = $1`, [messageId])
    }
}
