import { BaseClientCollection, BaseClientCollectionOptions } from './basecollection';
import { Session } from '../structures/session';
/**
 * @category Collection Options
 */
export interface SessionsOptions extends BaseClientCollectionOptions {
}
/**
 * Sessions Collection
 * (Bots cannot fill this)
 * @category Collections
 */
export declare class Sessions extends BaseClientCollection<string, Session> {
    insert(session: Session): void;
    get [Symbol.toStringTag](): string;
}
