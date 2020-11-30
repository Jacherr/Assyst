import { ShardClient } from '../client';
import { BaseSet } from '../collections/baseset';
import { StickerExtensions, StickerFormats } from '../constants';
import { UrlQuery } from '../utils';
import { BaseStructure, BaseStructureData } from './basestructure';
/**
 * Sticker Structure
 * @category Structure
 */
export declare class Sticker extends BaseStructure {
    readonly _keys: BaseSet<string>;
    asset: string;
    description: string;
    formatType: StickerFormats;
    id: string;
    name: string;
    packId: string;
    previewAsset: null | string;
    tags: null | string;
    constructor(client: ShardClient, data: BaseStructureData);
    get assetUrl(): string;
    get createdAt(): Date;
    get createdAtUnix(): number;
    get format(): StickerExtensions;
    assetUrlFormat(format?: null | string, query?: UrlQuery): string;
    toString(): string;
}
