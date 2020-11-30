/// <reference types="node" />
import { CryptoModules } from '../constants';
declare const _default: {
    readonly using: CryptoModules;
    readonly module: any;
    generateNonce(cache?: Buffer | null | undefined): Buffer;
    encrypt(key: Uint8Array, data: Buffer, nonce: Buffer, cache?: Buffer | null | undefined): {
        length: number;
        packet: Buffer;
    };
    decrypt(key: Uint8Array, data: Buffer, nonce: Buffer): Buffer | null;
};
export default _default;
