#version 430

in vec2 pos;
out vec4 color;

uniform uint screenWidth;
uniform uint screenHeight;

uniform uint screenCellsWidth;
uniform uint screenCellsHeight;

uniform usampler1D code_page;

uniform usampler1D chars;
uniform usampler1D fgs;
uniform usampler1D bgs;

const vec3 colours[16] = vec3[16](
    vec3(0.0, 0.0, 0.0),
    vec3(0.0, 0.0, 0.5),
    vec3(0.0, 0.5, 0.0),
    vec3(0.0, 0.5, 0.5),
    vec3(0.5, 0.0, 0.0),
    vec3(0.5, 0.0, 0.5),
    vec3(0.5, 0.5, 0.0),
    vec3(0.75, 0.75, 0.75),
    vec3(0.5, 0.5, 0.5),
    vec3(0.0, 0.0, 1.0),
    vec3(0.0, 1.0, 0.0),
    vec3(0.0, 1.0, 1.0),
    vec3(1.0, 0.0, 0.0),
    vec3(1.0, 0.0, 1.0),
    vec3(1.0, 1.0, 0.0),
    vec3(1.0, 1.0, 1.0)
);

vec3 getCodePagePixel(uint x, uint y, uint char, uint fg, uint bg) {
    uint bitIdx = (y % 4) * 8 + x;
    bool isFG = bool(texture(code_page, float(char * 2 + y / 4) / 512.0) & (1 << bitIdx));
    return isFG ? colours[fg] : colours[bg];
}

void main() {

    uint pixelX = uint((pos.x + 1.0) * float(screenWidth) / 2.0);
    uint pixelY = uint((pos.y + 1.0) * float(screenHeight) / 2.0);

    uint cellX = pixelX / 8;
    uint cellY = pixelY / 8;

    float numCells = float(screenCellsWidth * screenCellsHeight);
    uint cellIdx = cellX + cellY * screenCellsWidth;
    float cellIdxNorm = (float(cellIdx) + 0.5) / numCells;

    uint char = uint(texture(chars, cellIdxNorm)) << (24 - (cellIdx % 4) * 8) >> 24;
    uint fg = uint(texture(fgs, cellIdxNorm)) << (28 - (cellIdx % 8) * 4) >> 28;
    uint bg = uint(texture(bgs, cellIdxNorm)) << (28 - (cellIdx % 8) * 4) >> 28;

    color = vec4(getCodePagePixel(pixelX % 8, pixelY % 8, char, fg, bg), 1.0);

}