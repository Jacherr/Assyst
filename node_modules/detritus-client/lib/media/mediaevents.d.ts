import { User } from '../structures';
export declare namespace MediaEvents {
    interface ClientConnect {
        audioSSRC: number;
        user: null | User;
        userId: string;
        videoSSRC: number;
    }
    interface ClientDisconnect {
        user: null | User;
        userId: string;
    }
    interface Speaking {
        isSpeaking: boolean;
        priority: boolean;
        soundshare: boolean;
        ssrc: number;
        user: null | User;
        userId: string;
        voice: boolean;
    }
}
