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
