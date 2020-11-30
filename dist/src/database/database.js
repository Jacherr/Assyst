"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const pg_1 = require("pg");
const collections_1 = require("detritus-client/lib/collections");
var TableNames;
(function (TableNames) {
    TableNames["IMAGESCRIPT_PACKAGES"] = "is_packages";
    TableNames["IMAGESCRIPT_TAGS"] = "is_tags";
    TableNames["PREFIXES"] = "prefixes";
})(TableNames = exports.TableNames || (exports.TableNames = {}));
class Database {
    constructor(assyst, db) {
        this.imageScriptPackages = new collections_1.BaseCollection(Database.cacheOptions);
        this.imageScriptTags = new collections_1.BaseCollection(Database.cacheOptions);
        this.guildPrefixes = new collections_1.BaseCollection(Database.cacheOptions);
        this.assyst = assyst;
        this.db = new pg_1.Pool(db);
    }
    sql(query, values) {
        return __awaiter(this, void 0, void 0, function* () {
            return new Promise((resolve, reject) => {
                this.db.query(query, values || [], (err, res) => {
                    if (err)
                        reject(err);
                    else
                        resolve(res);
                });
            });
        });
    }
    getDatabaseSize() {
        return __awaiter(this, void 0, void 0, function* () {
            return this.sql(`select pg_size_pretty(pg_database_size('assyst'))`).then(r => r.rows[0]['pg_size_pretty']);
        });
    }
    createImageScriptPackage(name, content, owner) {
        return __awaiter(this, void 0, void 0, function* () {
            yield this.sql(`insert into ${TableNames.IMAGESCRIPT_PACKAGES}(name, content, owner) values($1, $2, $3)`, [name, content, owner]);
            this.imageScriptPackages.set(name, {
                name,
                content,
                owner
            });
        });
    }
    createImageScriptTag(name, content, owner) {
        return __awaiter(this, void 0, void 0, function* () {
            yield this.sql(`insert into ${TableNames.IMAGESCRIPT_TAGS}(name, content, owner, uses) values($1, $2, $3, 0)`, [name, content, owner]);
            this.imageScriptTags.set(name, {
                name,
                content,
                owner,
                uses: 0
            });
        });
    }
    deleteImageScriptPackage(name) {
        return __awaiter(this, void 0, void 0, function* () {
            yield this.sql(`delete from ${TableNames.IMAGESCRIPT_PACKAGES} where name = $1`, [name]);
            this.imageScriptPackages.delete(name);
        });
    }
    deleteImageScriptTag(name) {
        return __awaiter(this, void 0, void 0, function* () {
            yield this.sql(`delete from ${TableNames.IMAGESCRIPT_TAGS} where name = $1`, [name]);
            this.imageScriptTags.delete(name);
        });
    }
    editImageScriptPackage(name, content, owner) {
        return __awaiter(this, void 0, void 0, function* () {
            yield this.sql(`update ${TableNames.IMAGESCRIPT_PACKAGES} set content = $1 where name = $2`, [content, name]);
            this.imageScriptPackages.set(name, {
                name,
                content,
                owner
            });
        });
    }
    editImageScriptTag(name, content, owner, uses) {
        return __awaiter(this, void 0, void 0, function* () {
            yield this.sql(`update ${TableNames.IMAGESCRIPT_TAGS} set content = $1 where name = $2`, [content, name]);
            this.imageScriptTags.set(name, {
                name,
                content,
                owner,
                uses
            });
        });
    }
    fetchImageScriptPackage(name) {
        return __awaiter(this, void 0, void 0, function* () {
            let isPackage = this.imageScriptPackages.get(name);
            if (isPackage)
                return isPackage;
            isPackage = yield this.sql(`select * from ${TableNames.IMAGESCRIPT_PACKAGES} where name = $1`, [name]).then(r => r.rows[0]);
            if (isPackage)
                this.imageScriptPackages.set(name, isPackage);
            return isPackage;
        });
    }
    fetchImageScriptTag(name) {
        return __awaiter(this, void 0, void 0, function* () {
            let tag = this.imageScriptTags.get(name);
            if (tag)
                return tag;
            tag = yield this.sql(`select * from ${TableNames.IMAGESCRIPT_TAGS} where name = $1`, [name]).then(res => res.rows[0]);
            if (tag)
                this.imageScriptTags.set(name, tag);
            return tag;
        });
    }
    fetchUserImageScriptTags(owner) {
        return __awaiter(this, void 0, void 0, function* () {
            const tags = yield this.sql(`select * from ${TableNames.IMAGESCRIPT_TAGS} where owner = $1 order by uses desc`, [owner]).then(res => res.rows);
            for (const tag of tags) {
                this.imageScriptTags.set(tag.name, tag);
            }
            return tags;
        });
    }
    fetchTopImageScriptTags() {
        return __awaiter(this, void 0, void 0, function* () {
            const tags = yield this.sql(`select * from ${TableNames.IMAGESCRIPT_TAGS} order by uses desc limit 10`).then(r => r.rows);
            for (const tag of tags) {
                this.imageScriptTags.set(tag.name, tag);
            }
            return tags;
        });
    }
    fetchGuildPrefix(guildId) {
        return __awaiter(this, void 0, void 0, function* () {
            let prefix = this.guildPrefixes.get(guildId);
            if (prefix)
                return prefix;
            prefix = yield this.sql(`select prefix from ${TableNames.PREFIXES} where guild = $1`, [guildId]).then(res => res.rows[0] ? res.rows[0].prefix : undefined);
            if (prefix)
                this.guildPrefixes.set(guildId, prefix);
            return prefix;
        });
    }
    setGuildPrefix(guildId, prefix) {
        return __awaiter(this, void 0, void 0, function* () {
            this.guildPrefixes.set(guildId, prefix);
            this.sql(`insert into ${TableNames.PREFIXES}(guild, prefix) values($1, $2)`, [guildId, prefix]);
        });
    }
    editGuildPrefix(guildId, prefix) {
        return __awaiter(this, void 0, void 0, function* () {
            this.guildPrefixes.set(guildId, prefix);
            yield this.sql(`update ${TableNames.PREFIXES} set prefix = $1 where guild = $2`, [prefix, guildId]);
        });
    }
}
exports.Database = Database;
Database.cacheOptions = {
    expire: 60000,
    limit: 100
};
