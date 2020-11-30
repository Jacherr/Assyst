import { ShardClient } from '../client';
import { BaseCollection } from '../collections/basecollection';
import { BaseSet } from '../collections/baseset';
import { BaseStructure, BaseStructureData } from './basestructure';
import { ConnectedAccount } from './connectedaccount';
import { UserWithFlags } from './user';
/**
 * User Profile Structure
 * only non-bots will ever see these
 * @category Structure
 */
export declare class Profile extends BaseStructure {
    readonly _keys: BaseSet<string>;
    connectedAccounts: BaseCollection<string, ConnectedAccount>;
    mutualGuilds: BaseCollection<string, {
        id: string;
        nick: null | string;
    }>;
    nicks: BaseCollection<string, string>;
    premiumGuildSinceUnix: number;
    premiumSinceUnix: number;
    user: UserWithFlags;
    constructor(client: ShardClient, data: BaseStructureData);
    get premiumGuildSince(): Date | null;
    get premiumSince(): Date | null;
    mergeValue(key: string, value: any): void;
}
