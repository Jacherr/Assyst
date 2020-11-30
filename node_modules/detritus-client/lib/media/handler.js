"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.MediaGatewayOpHandler = exports.MediaHandler = void 0;
const constants_1 = require("../constants");
/**
 * Voice Connection Handler
 * @category Handler
 */
class MediaHandler {
    constructor(connection) {
        this.connection = connection;
        this.opHandler = new MediaGatewayOpHandler(this);
        this.gateway.on('packet', this.onPacket.bind(this));
        this.gateway.on('warn', this.connection.emit.bind(this.connection, 'warn'));
        this.gateway.once('killed', this.connection.kill.bind(this.connection));
        this.gateway.once('transportReady', this.onTransportReady.bind(this));
    }
    get client() {
        return this.connection.client;
    }
    get gateway() {
        return this.connection.gateway;
    }
    onPacket(packet) {
        if (packet.op in this.opHandler) {
            this.opHandler[packet.op](packet.d);
        }
    }
    onTransportReady(transport) {
        this.connection.emit('ready');
        transport.on('log', this.connection.emit.bind(this.connection, 'log'));
        transport.on('packet', this.onTransportPacket.bind(this));
        transport.on('warn', this.connection.emit.bind(this.connection, 'warn'));
    }
    onTransportPacket(packet) {
        this.connection.emit('packet', packet);
        let data = packet.data;
        try {
            switch (packet.format) {
                case constants_1.MediaCodecTypes.AUDIO:
                    {
                        if (!this.connection.decodeAudio) {
                            return;
                        }
                        if (packet.codec === constants_1.MediaCodecs.OPUS) {
                            if (!this.connection.opusDecoder) {
                                throw new Error('No Opus decoder provided');
                            }
                            if (packet.userId !== null) {
                                packet.data = this.connection.decode(packet.userId, data);
                            }
                        }
                    }
                    ;
                    break;
            }
            if (packet.format) {
                this.connection.emit(packet.format, packet);
            }
        }
        catch (error) {
            this.connection.emit('warn', error);
        }
    }
}
exports.MediaHandler = MediaHandler;
/**
 * Media Gateway Op Code Handler
 * @category Handlers
 */
class MediaGatewayOpHandler {
    constructor(handler) {
        this.handler = handler;
    }
    get client() {
        return this.handler.client;
    }
    get connection() {
        return this.handler.connection;
    }
    [constants_1.MediaOpCodes.CLIENT_CONNECT](data) {
        const payload = {
            audioSSRC: data['audio_ssrc'],
            user: this.client.users.get(data['user_id']) || null,
            userId: data['user_id'],
            videoSSRC: data['video_ssrc'],
        };
        this.connection.emit('connect', payload);
    }
    [constants_1.MediaOpCodes.CLIENT_DISCONNECT](data) {
        const userId = data['user_id'];
        if (this.connection.opusDecoders.has(userId)) {
            const opusDecoder = this.connection.opusDecoders.get(userId);
            opusDecoder.delete();
            this.connection.opusDecoders.delete(userId);
        }
        const user = this.client.users.get(userId) || null;
        const payload = { user, userId };
        this.connection.emit('disconnect', payload);
    }
    [constants_1.MediaOpCodes.SPEAKING](data) {
        const priority = (data['speaking'] & constants_1.SpeakingFlags.PRIORITY) === constants_1.SpeakingFlags.PRIORITY;
        const soundshare = (data['speaking'] & constants_1.SpeakingFlags.SOUNDSHARE) === constants_1.SpeakingFlags.SOUNDSHARE;
        const voice = (data['speaking'] & constants_1.SpeakingFlags.VOICE) === constants_1.SpeakingFlags.VOICE;
        const userId = data['user_id'];
        const payload = {
            isSpeaking: !!data['speaking'],
            priority,
            soundshare,
            ssrc: data['ssrc'],
            user: this.client.users.get(userId) || null,
            userId,
            voice,
        };
        this.connection.emit('speaking', payload);
    }
}
exports.MediaGatewayOpHandler = MediaGatewayOpHandler;
