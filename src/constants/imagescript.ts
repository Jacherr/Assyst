export const injectedCode: string = `Number.prototype.toRgba = function() {
    p = {};
    p.r = this >> 24 & 0xff;
    p.g = this >> 16 & 0xff;
    p.b = this >> 8 & 0xff;
    p.a = this & 0xff;
    return p;
}

Number.prototype.toHsla = function() {
    let p = {}
    let [r,g,b,a] = Object.values(this.toRgba()).map(a=>a / 255)
    let max = Math.max(...[r,g,b,a])
    let min = Math.min(...[r,g,b,a])
    if(max == r & g >= b) p.h = (1/6) * (g - b) / (max - min)
    if(max == r & g < b) p.h = (1/6) * (g - b) / (max - min) + 1
    if(max == g) p.h = (1/6) * (b - r) / (max - min) + 2/6
    if(max == b) p.h = (1/6) * (r - g) / (max - min) + 4/6
    p.s = (max - min) / (1 - Math.abs(1 - (max + min)))
    p.l = (max + min) / 2
    p.a = a
    return p
};

const random = (min,max) => max ? Math.round(Math.random() * (max - min) + min) : Math.round(Math.random() * min);

const checkBounds = (x, y, w, h) => x >= 1 && y >= 1 && x <= w && y <= h
`;
