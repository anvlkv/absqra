import { Expr } from '..';


export class Program <T extends Expr = any> {
    public root: T = null;
    public children: Program[] = [];
}