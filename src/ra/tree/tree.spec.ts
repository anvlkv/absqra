import { Tree } from './tree';
import { expect } from 'chai';


describe('Tree', () => {
    let tree: Tree<string>;
    before(() => {
        tree = new Tree('root');
        tree.branch('child1'); // 0 // swap 1
        tree.addAt([0], 'child0'); // swap 0
    });

    it('should add children', () => {
        expect(tree.children[0].root).to.be.equal('child0');
    });

    it('should add children at', () => {
        expect(tree.children[1].root).to.be.equal('child1')
    });

    context('with more levels', () => {
        before(() => {
            let child = tree.branch('child2'); //
            child.branch('child3');
            child.branch('child4');
            child = child.branch('child5');
            child.branch('child6');
            tree.branch('child7');
            tree.addAt([2, 2, 0], 'child5.5');
        });

        it('should flatten tree', () => {
            expect(tree.flat()).to.be.deep.equal(['root', 'child0', 'child1', 'child2', 'child3', 'child4', 'child5', 'child5.5', 'child6', 'child7']);
        });

        it('should treeMap children', () => {
            expect(tree.treeMap()).to.be.deep.equal([[], [0], [1], [2], [2, 0], [2, 1], [2, 2], [2, 2, 0], [2, 2, 1], [3]]);
        });

        it('should treeMap and flatten in same order', () => {
            const map = tree.treeMap();
            const flat = tree.flat();
            map.forEach((address, i) => {
                expect(tree.getAt(address), `${address.join(':')}`).to.be.equal(flat[i]);
            });
        });
    });


});