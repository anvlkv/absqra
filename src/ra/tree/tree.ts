export class Tree<T> {

    constructor(
        public root: T = null,
        public children: Tree<T>[] = []
    ) {}

    public flat(): T[] {
        return this.children.reduce((flat, child, at, all) => {
            if (child.children) {
                flat.push(...child.flat())
            }
            return flat;
        }, [this.root]);
    }

    public treeMap(): number[][] {
        return this.children.reduce((map, child, at, all) => {
            const address = [[at]];
            if (child.children && child.children.length) {
                address.splice(0, 1, ...child.treeMap().map((ad, i) => address[0].concat(ad)));
            }
            return map.concat(address);
        }, [[]]);
    }

    public getAt(at: number[]): T {
        if (at.length) {
            return this.getBranchAt(at).root;
        }
        else {
            return this.root;
        }
    }

    public addAt(at: number[], val: T) {
        const branch = this.getBranchAt(at.slice(0, at.length - 1)) || this;
        branch.children.splice(at[at.length - 1], 0, new Tree<T>(val));
    }

    public branch(val: T): Tree<T> {
        this.children.push(new Tree<T>(val));
        return this.children[this.children.length - 1];
    }

    public getBranchAt(at: number[]): Tree<T> {
        let branch = this.children;
        let it = 0;
        while (at.length > it + 1) {
            let b = branch[at.slice(it, it + 1)[0]];
            if  (b) {
                branch = b.children;
            }
            else {
                throw new Error(`No such [${this.constructor.name}-tree] branch: [${at.join(':')}]`)
            }
            it++;
        }
        return branch[at[at.length - 1]];
    }
}