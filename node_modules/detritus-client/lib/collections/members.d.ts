import { BaseClientCollectionOptions, BaseClientGuildReferenceCache } from './basecollection';
import { Member } from '../structures';
/**
 * @category Collection Options
 */
export interface MembersOptions extends BaseClientCollectionOptions {
}
/**
 * Members Collection
 * @category Collections
 */
export declare class Members extends BaseClientGuildReferenceCache<string, Member> {
    key: string;
    insert(member: Member): void;
    get [Symbol.toStringTag](): string;
}
