export declare const ValidEndianness: Readonly<{
    BE: string;
    LE: string;
}>;
export interface AudioFormatOptions {
    bitDepth?: number;
    channels?: number;
    endianness?: string;
    frameDuration?: number;
    sampleRate?: number;
}
export declare class AudioFormat {
    bitDepth: number;
    channels: number;
    endianness: string;
    frameDuration: number;
    sampleRate: number;
    constructor(options?: AudioFormatOptions);
    get byteDepth(): number;
    get frameSize(): number;
    get samplesPerFrame(): number;
    get samplesPerTick(): number;
    get sampleSize(): number;
    get pcmMult(): number;
    get pcmMax(): number;
    get pcmMin(): number;
    get readFunc(): string;
    get writeFunc(): string;
}
