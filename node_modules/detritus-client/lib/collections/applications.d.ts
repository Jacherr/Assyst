import { BaseClientCollection, BaseClientCollectionOptions } from './basecollection';
import { Application } from '../structures/application';
export interface ApplicationsOptions extends BaseClientCollectionOptions {
}
/**
 * Applications Collection
 * @category Collections
 */
export declare class Applications extends BaseClientCollection<string, Application> {
    lastRefresh: number;
    refreshTime: number;
    get shouldRefresh(): boolean;
    insert(application: Application): void;
    fill(applications?: Array<any>): Promise<void>;
    get [Symbol.toStringTag](): string;
}
