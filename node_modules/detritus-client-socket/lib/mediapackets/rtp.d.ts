/// <reference types="node" />
export declare const ValidRTPVersion = 2;
export declare function isValidRTPHeader(buffer: Buffer): boolean;
export declare class RTPHeader {
    buffer: Buffer;
    nonce?: Buffer;
    payload?: Buffer;
    constructor(options?: {
        buffer?: Buffer;
        marker?: boolean;
        payloadType?: number;
        randomize?: boolean;
        ssrc?: number;
        version?: number;
    });
    get length(): number;
    get valid(): boolean;
    get firstByte(): number;
    get secondByte(): number;
    get version(): number;
    get padding(): number;
    get extension(): number;
    get csrcCount(): number;
    get marker(): number;
    get payloadType(): number;
    get sequence(): number;
    get timestamp(): number;
    get ssrc(): number;
    get nonceNumber(): number;
    randomizeSequence(): void;
    randomizeTimestamp(): void;
    randomizeNonce(): void;
    setVersion(version: number): void;
    setPadding(padding: boolean | number): void;
    setExtension(extension: boolean | number): void;
    setCSRCCount(csrcCount: number): void;
    setMarker(marker: boolean | number): void;
    setPayloadType(payloadType: number): void;
    setSequence(sequence?: number, increment?: boolean): void;
    setTimestamp(timestamp?: number, increment?: boolean): void;
    setSSRC(ssrc: number): void;
    setPayload(payload: Buffer, replace?: boolean): void;
    setNonce(nonce?: Buffer | number, increment?: boolean): void;
    reset(): void;
    copy(target: Buffer, targetStart?: number, sourceStart?: number, sourceEnd?: number): number;
}
export declare class RTPNonce {
    buffer: Buffer;
    constructor(options?: {
        randomize?: boolean;
    });
    get number(): number;
    copy(target: Buffer, targetStart?: number, sourceStart?: number, sourceEnd?: number): number;
    generate(): Buffer;
    randomize(): Buffer;
    set(nonce?: number, increment?: boolean): Buffer;
}
