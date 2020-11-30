import { ShardClient } from '../client';
import { BaseSet } from '../collections/baseset';
import { BaseStructure, BaseStructureData } from './basestructure';
import { User } from './user';
/**
 * Guild Template Structure
 * @category Structure
 */
export declare class Template extends BaseStructure {
    readonly _keys: BaseSet<string>;
    readonly _keysMerge: BaseSet<string>;
    code: string;
    createdAt: Date;
    creator: User;
    creatorId: string;
    description: string;
    isDirty: boolean;
    name: string;
    serializedSourceGuild?: any;
    sourceGuildId: string;
    updatedAt: Date;
    usageCount: number;
    constructor(client: ShardClient, data: BaseStructureData);
    get createdAtUnix(): number;
    get isUpdated(): boolean;
    get longUrl(): string;
    get updatedAtUnix(): number;
    get url(): string;
    delete(): Promise<any>;
    fetch(): Promise<Template>;
    mergeValue(key: string, value: any): void;
}
