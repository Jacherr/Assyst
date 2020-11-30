import { ShardClient } from '../client';
import { BaseCollection } from '../collections/basecollection';
import { BaseSet } from '../collections/baseset';
import { BaseStructure, BaseStructureData } from './basestructure';
import { PresenceActivity } from './presence';
/**
 * Discord Session Structure (Users Only)
 * @category Structure
 */
export declare class Session extends BaseStructure {
    readonly _keys: BaseSet<string>;
    readonly _keysMerge: BaseSet<string>;
    _activities?: BaseCollection<string, PresenceActivity>;
    active: boolean;
    clientInfo: SessionClientInfo;
    sessionId: string;
    status: string;
    constructor(client: ShardClient, data: BaseStructureData);
    get activity(): null | PresenceActivity;
    get activities(): BaseCollection<string, PresenceActivity>;
    get game(): null | PresenceActivity;
    get isDnd(): boolean;
    get isIdle(): boolean;
    get isOffline(): boolean;
    get isOnline(): boolean;
    mergeValue(key: string, value: any): void;
}
/**
 * Session Client Info Structure, used in [Session]
 * @category Structure
 */
export declare class SessionClientInfo extends BaseStructure {
    readonly _keys: BaseSet<string>;
    readonly _keysMerge: BaseSet<string>;
    readonly session: Session;
    clientString: string;
    os: string;
    version: number;
    constructor(session: Session, data: BaseStructureData);
    mergeValue(key: string, value: any): void;
    toJSON(): any;
}
