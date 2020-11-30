import { ShardClient } from '../client';
import { BaseSet } from '../collections/baseset';
import { PlatformTypes } from '../constants';
import { BaseStructure, BaseStructureData } from './basestructure';
/**
 * Connected Account
 * If from a user profile, it'll be partial
 * @category Structure
 */
export declare class ConnectedAccount extends BaseStructure {
    readonly _keys: BaseSet<string>;
    accessToken?: string;
    friendSync?: boolean;
    id: string;
    integrations?: Array<any>;
    name: string;
    revoked?: boolean;
    showActivity?: boolean;
    type: PlatformTypes;
    verified: boolean;
    visibility?: number;
    constructor(client: ShardClient, data: BaseStructureData);
    get key(): string;
}
