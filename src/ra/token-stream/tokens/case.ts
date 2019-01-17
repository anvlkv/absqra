import { Token, TokenType } from '../token';
import { LineColumnAddress } from '../../line-column-address';


export class CaseToken extends Token {
    constructor(start: LineColumnAddress, end: LineColumnAddress){
        super(
            TokenType.KW,
            'case',
            start,
            end
        );
    }
}