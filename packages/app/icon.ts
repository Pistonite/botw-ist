scale(1);
unit(1);

const color_core1 = "#F0C83D83";
const color_core2 = "#FE8F0088";
const color_core3 = "red";
const color_shell1 = "#AD8867";
const color_shell2 = "#5C4D48";
const color_shell3 = "#E7DED5";
const color_light = "#73FBFD";

function render_core(t: Point) {
    const pillar = point(0, 0, 0).sized(14, 6, 6);
    const pillar2 = pillar.translated("y", 8).union(pillar);
    const pillar4 = pillar2.translated("z", 8).union(pillar2);
    const pillar_cut = point(0, 3, 3).sized(14, 8, 8);
    const side_cut = point(3, 0, 3).sized(8, 14, 8);
    const top_cut = point(3, 3, 0).sized(8, 8, 14);

    const corner = point(0, 0, 0).sized(2, 2, 2);
    const corner2 = corner.translated("x", 12).union(corner);
    const corner4 = corner2.translated("y", 12).union(corner2);
    const corner8 = corner4.translated("z", 12).union(corner4);

    const core = point(1, 1, 1).sized(12, 12, 12);
    const core_in = point(2, 2, 2).sized(10, 10, 10);
    const core_in2 = point(3, 3, 3).sized(8, 8, 8);

    const center_pillar_out = point(0, 5, 5).sized(14, 4, 4);
    const center_pillar_in = point(0, 6, 6).sized(14, 2, 2);

    center_pillar_out
        .difference(center_pillar_in)
        .difference(core)
        .translated(t)
        .render(color_shell3);

    center_pillar_in.difference(core).translated(t).render(color_light);

    pillar4
        .difference(corner8)
        .difference(pillar_cut)
        .difference(side_cut)
        .difference(top_cut)
        .difference(core)
        .translated(t)
        .render(color_shell2);

    core.difference(corner8).translated(t).render(color_core1);

    core_in.translated(t).render(color_core2);
    core_in2.translated(t).render(color_core3);

    core.intersection(corner8).translated(t).render(color_shell2);
}

function render_ring(x: number, width: number) {
    const center_cut = point(0, 1, 1).sized(14, 16, 16);
    const middle_ring = point(x, 0, 0).sized(width, 18, 18);

    middle_ring.difference(center_cut).render(color_light);
}

function render_hring() {
    const center_cut = point(1, 1, 1).sized(16, 18, 18);
    const ring = point(0, 0, 7).sized(18, 20, 3);
    const ring2 = point(0, 0, 8).sized(18, 20, 1);

    ring.difference(center_cut)
        //.difference(ring2)
        .translated(-1, 0, 0)
        .render(color_shell1);

    // ring2
    //     .difference(center_cut)
    //     .translated(-1, 0, 0)
    //     .render(color_shell2)
}

render_core(point(0, 2, 2));
//render_ring(2, 1)
render_ring(6, 2);
//render_ring(11, 1)
render_hring();
