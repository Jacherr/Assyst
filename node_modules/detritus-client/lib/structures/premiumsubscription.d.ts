import { ShardClient } from '../client';
import { BaseSet } from '../collections/baseset';
import { BaseStructure, BaseStructureData } from './basestructure';
import { Guild } from './guild';
import { User } from './user';
/**
 * Premium Subscription Structure, details a user's nitro boost on the server
 * ATM, only non-bots will ever see these
 * @category Structure
 */
export declare class PremiumSubscription extends BaseStructure {
    readonly _keys: BaseSet<string>;
    ended: boolean;
    guildId: string;
    id: string;
    userId: string;
    constructor(client: ShardClient, data: BaseStructureData);
    get createdAt(): Date;
    get createdAtUnix(): number;
    get guild(): Guild | null;
    get user(): User | null;
}
