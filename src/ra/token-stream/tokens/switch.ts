import { Token, TokenType } from '../token';
import { LineColumnAddress } from '../../line-column-address';


export class SwitchToken extends Token {
    constructor(start: LineColumnAddress, end: LineColumnAddress){
        super(
            TokenType.KW,
            'switch',
            start,
            end
        );
    }
}