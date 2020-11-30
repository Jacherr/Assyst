"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const constants_1 = require("../constants");
const PCrypto = {
    available: {},
    modules: [
        constants_1.CryptoModules.SODIUM,
        constants_1.CryptoModules.LIBSODIUM_WRAPPERS,
        constants_1.CryptoModules.TWEETNACL,
    ],
    using: null,
};
// order in preference
(async () => {
    for (let name of PCrypto.modules) {
        try {
            const crypto = PCrypto.available[name] = require(name);
            switch (name) {
                case constants_1.CryptoModules.LIBSODIUM_WRAPPERS:
                    {
                        if (crypto.ready) {
                            await crypto.ready;
                        }
                    }
                    ;
                    break;
            }
            break;
        }
        catch (error) {
            continue;
        }
    }
    PCrypto.using = PCrypto.modules.find((mod) => (mod in PCrypto.available)) || null;
})();
function Uint8ArrayToBuffer(array, cache) {
    if (cache) {
        for (let i = 0; i < array.length; i++) {
            cache[i] = array[i];
        }
        return cache;
    }
    return Buffer.from(array);
}
exports.default = {
    get using() {
        if (!PCrypto.using) {
            throw new Error(`For media (video/voice) packing/unpacking, please install one of: ${JSON.stringify(PCrypto.modules)}`);
        }
        return PCrypto.using;
    },
    get module() {
        const crypto = PCrypto.available[this.using];
        switch (this.using) {
            case constants_1.CryptoModules.SODIUM:
                {
                    return crypto.api;
                }
                ;
        }
        return crypto;
    },
    generateNonce(cache) {
        const crypto = this.module;
        let nonce;
        switch (this.using) {
            case constants_1.CryptoModules.LIBSODIUM_WRAPPERS:
                {
                    const generated = crypto.randombytes_buf(crypto.crypto_secretbox_NONCEBYTES);
                    nonce = Uint8ArrayToBuffer(generated, cache);
                }
                ;
                break;
            case constants_1.CryptoModules.SODIUM:
                {
                    nonce = cache || Buffer.alloc(crypto.crypto_secretbox_NONCEBYTES);
                    crypto.randombytes_buf(nonce);
                }
                ;
                break;
            case constants_1.CryptoModules.TWEETNACL:
                {
                    const generated = crypto.randomBytes(crypto.box.nonceLength);
                    nonce = Uint8ArrayToBuffer(generated, cache);
                }
                ;
                break;
            default:
                {
                    throw new Error(`For media (video/voice) packing/unpacking, please install one of: ${JSON.stringify(PCrypto.modules)}`);
                }
                ;
        }
        return nonce;
    },
    encrypt(key, data, nonce, cache) {
        const crypto = this.module;
        let length = 0;
        let packet;
        switch (this.using) {
            case constants_1.CryptoModules.LIBSODIUM_WRAPPERS:
                {
                    length += data.length + crypto.crypto_secretbox_MACBYTES;
                    if (cache) {
                        cache.fill(0, 0, length);
                    }
                    const generated = crypto.crypto_secretbox_easy(data, nonce, key);
                    packet = Uint8ArrayToBuffer(generated, cache);
                }
                ;
                break;
            case constants_1.CryptoModules.SODIUM:
                {
                    length += data.length + crypto.crypto_secretbox_MACBYTES;
                    if (cache) {
                        cache.fill(0, 0, length);
                    }
                    packet = crypto.crypto_secretbox_easy(data, nonce, key);
                    if (cache) {
                        packet.copy(cache);
                        packet = cache;
                    }
                }
                ;
                break;
            case constants_1.CryptoModules.TWEETNACL:
                {
                    length += data.length + crypto.secretbox.overheadLength;
                    const generated = crypto.secretbox(data, nonce, key);
                    packet = Uint8ArrayToBuffer(generated, cache);
                }
                ;
                break;
            default:
                {
                    throw new Error(`For media (video/voice) packing/unpacking, please install one of: ${JSON.stringify(PCrypto.modules)}`);
                }
                ;
        }
        return { length, packet };
    },
    decrypt(key, data, nonce) {
        const crypto = this.module;
        let packet = null;
        switch (this.using) {
            case constants_1.CryptoModules.LIBSODIUM_WRAPPERS:
                {
                    const generated = crypto.crypto_secretbox_open_easy(data, nonce, key);
                    if (generated) {
                        packet = Uint8ArrayToBuffer(generated);
                    }
                }
                ;
                break;
            case constants_1.CryptoModules.SODIUM:
                {
                    packet = crypto.crypto_secretbox_open_easy(data, nonce, key);
                }
                ;
                break;
            case constants_1.CryptoModules.TWEETNACL:
                {
                    const generated = crypto.secretbox.open(data, nonce, key);
                    if (generated) {
                        packet = Uint8ArrayToBuffer(generated);
                    }
                }
                ;
                break;
        }
        return packet;
    },
};
