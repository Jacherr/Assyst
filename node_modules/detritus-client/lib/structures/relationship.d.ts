import { ShardClient } from '../client';
import { BaseSet } from '../collections/baseset';
import { RelationshipTypes } from '../constants';
import { BaseStructure, BaseStructureData } from './basestructure';
import { User } from './user';
/**
 * Relationship Structure
 * Used to describe a relationship with a user
 * (only non-bots)
 * @category Structure
 */
export declare class Relationship extends BaseStructure {
    readonly _keys: BaseSet<string>;
    id: string;
    type: RelationshipTypes;
    user: User;
    constructor(client: ShardClient, data: BaseStructureData);
    get isBlocked(): boolean;
    get isFriend(): boolean;
    get isImplicit(): boolean;
    get isNone(): boolean;
    get isPendingIncoming(): boolean;
    get isPendingOutgoing(): boolean;
    mergeValue(key: string, value: any): void;
}
