export declare namespace MediaRawEvents {
    interface MediaGatewayPacket {
        op: number;
        d: any;
    }
    interface ClientConnect {
        audio_ssrc: number;
        user_id: string;
        video_ssrc: number;
    }
    interface ClientDisconnect {
        user_id: string;
    }
    interface Speaking {
        speaking: number;
        ssrc: number;
        user_id: string;
    }
}
