export class Tree<T> {

    constructor(
        public root: T,
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

    public getAt(at: number[]): T {
        return this.branchAt(at).root;
    }

    public addAt(at: number[], val: T) {
        const branch = this.branchAt(at.slice(0, at.length - 1));
        branch.children.splice(at[at.length - 1], 0, new Tree<T>(val));
    }

    private branchAt(at: number[]) {
        let branch = this.children;
        while (at.length > 1) {
            branch = branch[at.splice(0, 1)[0]].children
        }
        return branch[at[0]];
    }
}