function logGLCall(functionName: string, args: unknown[]) {
    console.log(
        'gl.' +
            functionName +
            '(' +
            window.WebGLDebugUtils.glFunctionArgsToString(functionName, args) +
            ')'
    );
}

export function initializeWebGlContext(debug?: boolean) {
    const canvas = document.querySelector('canvas');
    const glContext = canvas?.getContext('webgl2')
    const gl = debug ? window.WebGLDebugUtils.makeDebugContext(
        glContext,
        undefined,
        logGLCall
    ) : glContext;

    if (!gl) {
        throw new Error('Failed to initialize WebGL context.');
    }

    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT);

    return gl;
}

export function loadShader(gl: WebGL2RenderingContext, type: number, source: string) {
    const shader = gl.createShader(type);

    if (!shader) {
        return;
    }

    gl.shaderSource(shader, source);
    gl.compileShader(shader);

    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        console.error('Failed to compile shaders: ' + gl.getShaderInfoLog(shader));
        gl.deleteShader(shader);
        return;
    }

    return shader;
}

export function createShader(gl: WebGL2RenderingContext, type: number, source: string) {
    const shader = gl.createShader(type);

    if (!shader) {
        return;
    }

    gl.shaderSource(shader, source);
    gl.compileShader(shader);

    const success = gl.getShaderParameter(shader, gl.COMPILE_STATUS);

    if (success) {
        return shader;
    }

    console.log(gl.getShaderInfoLog(shader));
    gl.deleteShader(shader);
}

export function createProgram(gl: WebGL2RenderingContext, shaders: WebGLShader[]) {
    const program = gl.createProgram();

    if (!program) {
        return;
    }

    for (const shader of shaders) {
        gl.attachShader(program, shader);
    }

    gl.linkProgram(program);

    const success = gl.getProgramParameter(program, gl.LINK_STATUS);

    if (success) {
        return program;
    }

    console.log(gl.getProgramInfoLog(program));
    gl.deleteShader(program);
}

export const m3 = {
    multiply: function(a: number[], b: number[]) {
        const a00 = a[0 * 3 + 0];
        const a01 = a[0 * 3 + 1];
        const a02 = a[0 * 3 + 2];
        const a10 = a[1 * 3 + 0];
        const a11 = a[1 * 3 + 1];
        const a12 = a[1 * 3 + 2];
        const a20 = a[2 * 3 + 0];
        const a21 = a[2 * 3 + 1];
        const a22 = a[2 * 3 + 2];
        const b00 = b[0 * 3 + 0];
        const b01 = b[0 * 3 + 1];
        const b02 = b[0 * 3 + 2];
        const b10 = b[1 * 3 + 0];
        const b11 = b[1 * 3 + 1];
        const b12 = b[1 * 3 + 2];
        const b20 = b[2 * 3 + 0];
        const b21 = b[2 * 3 + 1];
        const b22 = b[2 * 3 + 2];
     
        return [
          b00 * a00 + b01 * a10 + b02 * a20,
          b00 * a01 + b01 * a11 + b02 * a21,
          b00 * a02 + b01 * a12 + b02 * a22,
          b10 * a00 + b11 * a10 + b12 * a20,
          b10 * a01 + b11 * a11 + b12 * a21,
          b10 * a02 + b11 * a12 + b12 * a22,
          b20 * a00 + b21 * a10 + b22 * a20,
          b20 * a01 + b21 * a11 + b22 * a21,
          b20 * a02 + b21 * a12 + b22 * a22,
        ];
      },

    translate: function(dx: number, dy: number) {
        // prettier-ignore
        return [
            1, 0, 0,
            0, 1, 0,
            dx, dy, 1,
        ]
    },

    rotate: function(angle: number) {
        // prettier-ignore
        return [
            Math.cos(angle), -Math.sin(angle), 0,
            Math.sin(angle), Math.cos(angle), 0,
            0, 0, 0
        ]
    },

    scale: function(sx: number, sy: number) {
        // prettier-ignore
        return [
            sx, 0, 0,
            0, sy, 0,
            0, 0, 0
        ]
    }
}