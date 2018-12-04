export type LineColumnAddress = [number, number];

export class LineColumn {
    static isAhead([l1, c1]: LineColumnAddress, [l2, c2]: LineColumnAddress): boolean {
        let result = false;
        if (l1 > l2) {
            result = true;
        }
        else if (l1 === l2 && c1 > c2) {
            result = true;
        }
        return result;
    }

    static isBehind([l1, c1]: LineColumnAddress, [l2, c2]: LineColumnAddress): boolean {
        let result = false;
        if (l1 < l2) {
            result = true;
        }
        else if (l1 === l2 && c1 < c2) {
            result = true;
        }
        return result;
    }
}