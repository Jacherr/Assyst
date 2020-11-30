"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.InflateError = exports.MediaRTPError = exports.MediaPacketError = exports.SocketKillError = exports.DroppedPacketError = void 0;
class BaseError extends Error {
}
class DroppedPacketError extends BaseError {
    constructor(packet, message) {
        let errorMessage = 'Packet dropped';
        if (message) {
            errorMessage += `, reason: (${message})`;
        }
        super(errorMessage);
        this.packet = packet;
    }
}
exports.DroppedPacketError = DroppedPacketError;
class SocketKillError extends BaseError {
    constructor(code, reason) {
        let message;
        if (reason) {
            message = `Socket closed with ${code} (${reason}), killing.`;
        }
        else {
            message = `Socket closed with ${code}, killing.`;
        }
        super(message);
        this.code = code;
        this.reason = reason || null;
    }
}
exports.SocketKillError = SocketKillError;
class MediaPacketError extends BaseError {
    constructor(message, from, packet) {
        super(message);
        this.from = from;
        this.packet = packet;
    }
}
exports.MediaPacketError = MediaPacketError;
class MediaRTPError extends MediaPacketError {
    constructor(message, from, packet, rtp) {
        super(message, from, packet);
        this.rtp = rtp;
    }
}
exports.MediaRTPError = MediaRTPError;
class InflateError extends BaseError {
    constructor(message, code) {
        super(message);
        this.code = code;
    }
}
exports.InflateError = InflateError;
