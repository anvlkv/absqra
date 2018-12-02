import { LineColumnAddress } from '../block-stream/block';


export class RaLine {
    constructor(
        public readonly indent: number,
        public readonly value: string,
        public start: LineColumnAddress,
        public end: LineColumnAddress
    ) {}
}