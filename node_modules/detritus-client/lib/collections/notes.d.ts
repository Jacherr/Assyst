import { BaseClientCollection, BaseClientCollectionOptions } from './basecollection';
/**
 * @category Collection Options
 */
export interface NotesOptions extends BaseClientCollectionOptions {
}
/**
 * Notes Collection
 * (Bots cannot fill this)
 * @category Collections
 */
export declare class Notes extends BaseClientCollection<string, string> {
    insert(userId: string, note: string): void;
    get [Symbol.toStringTag](): string;
}
