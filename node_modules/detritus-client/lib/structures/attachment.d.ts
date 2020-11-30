import { BaseSet } from '../collections/baseset';
import { BaseStructure, BaseStructureData } from './basestructure';
import { Message } from './message';
export declare const EmbeddableRegexes: Readonly<{
    audio: RegExp;
    image: RegExp;
    video: RegExp;
}>;
export declare const MimeClassTypes: Array<{
    classType: string;
    regex: RegExp;
    type: string;
}>;
/**
 * Attachment Structure, used for [Message] objects
 * @category Structure
 */
export declare class Attachment extends BaseStructure {
    readonly _keys: BaseSet<string>;
    readonly message: Message;
    filename: string;
    height: number;
    id: string;
    proxyUrl?: string;
    size: number;
    url?: string;
    width: number;
    constructor(message: Message, data: BaseStructureData);
    get classType(): string;
    get createdAt(): Date;
    get createdAtUnix(): number;
    get extension(): string;
    get hasSpoiler(): boolean;
    get isAudio(): boolean;
    get isImage(): boolean;
    get isVideo(): boolean;
    get isEmbeddable(): boolean;
    get mimetype(): string;
    toString(): string;
}
