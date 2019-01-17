import { Token, TokenType } from '../token';
import { LineColumnAddress } from '../../line-column-address';


export class LnToken extends Token {
    constructor(start: LineColumnAddress, end: LineColumnAddress){
        super(
            TokenType.KW,
            'ln',
            start,
            end
        );
    }
}