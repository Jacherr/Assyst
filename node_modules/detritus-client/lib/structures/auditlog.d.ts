import { ShardClient } from '../client';
import { BaseCollection } from '../collections/basecollection';
import { BaseSet } from '../collections/baseset';
import { AuditLogActions, AuditLogChangeKeys } from '../constants';
import { BaseStructure, BaseStructureData } from './basestructure';
import { ChannelGuildBase } from './channel';
import { Guild } from './guild';
import { Role } from './role';
import { User } from './user';
import { Webhook } from './webhook';
/**
 * Guild Audit Log
 * @category Structure
 */
export declare class AuditLog extends BaseStructure {
    readonly _keys: BaseSet<string>;
    actionType: AuditLogActions;
    changes: BaseCollection<string, any>;
    id: string;
    guildId: string;
    options?: AuditLogOptions;
    reason?: string;
    target?: User | Webhook;
    targetId?: string;
    user?: User;
    userId?: string;
    constructor(client: ShardClient, data: BaseStructureData);
    get createdAt(): Date;
    get createdAtUnix(): number;
    get guild(): Guild | null;
    mergeValue(key: string, value: any): void;
}
/**
 * Guild Audit Log Change, used in [[AuditLog]]
 * @category Structure
 */
export declare class AuditLogChange extends BaseStructure {
    readonly _keys: BaseSet<string>;
    readonly log: AuditLog;
    key: AuditLogChangeKeys;
    newValue: any;
    oldValue: any;
    constructor(log: AuditLog, data: BaseStructureData);
}
/**
 * Guild Audit Log Options, used in [[AuditLog]]
 * @category Structure
 */
export declare class AuditLogOptions extends BaseStructure {
    readonly _keys: BaseSet<string>;
    readonly log: AuditLog;
    channel?: ChannelGuildBase;
    channelId?: string;
    count?: number;
    deleteMemberDays?: number;
    id?: string;
    membersRemoved?: number;
    subtarget?: Role | User;
    type?: number;
    constructor(log: AuditLog, data: BaseStructureData);
    mergeValue(key: string, value: any): void;
}
