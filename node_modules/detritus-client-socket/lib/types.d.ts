export declare namespace GatewayPackets {
    interface Packet {
        d: any;
        op: number;
        s: number;
        t: string;
    }
    interface Hello {
        _trace: Array<string>;
        heartbeat_interval: number;
    }
    type Heartbeat = null;
    type HeartbeatAck = null;
    type InvalidSession = boolean;
    type Reconnect = null;
    namespace DispatchEvents {
        interface GuildDelete {
            id: string;
            unavailable: boolean;
        }
        interface VoiceServerUpdate {
            channel_id: string;
            endpoint: string;
            guild_id?: string;
            token: string;
        }
        interface VoiceStateUpdate {
            channel_id: string;
            guild_id?: string;
            session_id: string;
            user_id: string;
        }
    }
}
export declare namespace MediaGatewayPackets {
    interface Packet {
        d: any;
        op: number;
    }
    interface ClientConnect {
        audio_ssrc: number;
        user_id: string;
        video_ssrc?: number;
    }
    interface ClientDisconnect {
        user_id: string;
    }
    interface Hello {
        heartbeat_interval: number;
        v: number;
    }
    type HeartbeatAck = number;
    interface Ready {
        experiments: Array<string>;
        ip: string;
        port: number;
        modes: Array<string>;
        ssrc: number;
    }
    interface Resumed {
    }
    interface SelectProtocolAckUDP {
        audio_codec: string;
        mode: string;
        media_session_id: string;
        secret_key: Array<number>;
        video_codec: string;
    }
    interface SelectProtocolAckWebRTC {
        audio_codec: string;
        media_session_id: string;
        sdp: string;
        video_codec: string;
    }
    interface SessionUpdate {
        audio_codec?: string;
        media_session_id: string;
        video_codec?: string;
        video_quality_changes?: Array<{
            quality: string;
            ssrc: number;
            user_id: string;
        }>;
    }
    interface Speaking {
        speaking: number;
        ssrc: number;
        user_id: string;
    }
    interface VideoSinkWants {
        any: number;
    }
}
